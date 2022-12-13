use super::{selector::SuumoSelector, SearchQueryParams, Transfers};
use crate::{
    env::get_env_var,
    repository::{crawler::HttpClient, HtmlParser},
};
use anyhow::{bail, Context, Result};
use domain::model::{Residence, ResidenceHeader, Residences, Room, TargetArea};
use reqwest::Url;

#[async_trait::async_trait]
pub trait SuumoCrawler: HttpClient + HtmlParser + SuumoSelector {
    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(skip_all, err(Debug))]
    async fn health_check(&self) -> Result<()> {
        // suumo関東版のトップページを取得する
        let url = get_env_var("URL_SUUMO_KANTO_DOMAIN").unwrap();
        let res = self.client().get(&url).send().await?;

        // トップページのh1テキストを読む
        let text = res.text().await?;
        let top_kanto_title = self.innter_text_of_element(&text, self.kanto_title(), ",");
        tracing::info!("{}", top_kanto_title);

        // テキスト内容のチェック
        if top_kanto_title == "関東の住宅・不動産情報探し" {
            Ok(())
        } else {
            bail!("Unknown text: {}", text)
        }
    }

    /// 検索条件を指定して賃貸一覧ページのURLを取得する
    #[tracing::instrument(skip_all, fields(area=area.to_string(), station=station), err(Debug))]
    async fn url_of_room_list(&self, area: TargetArea, station: &str) -> Result<Url> {
        // 検索条件を指定したクエリパラメータを設定する
        let query = SearchQueryParams::builder()
            .最寄り駅(station)
            .駅徒歩("20")?
            .電車での所要時間("60")?
            .乗り換え回数(Transfers::こだわらない)
            .住みたいエリア(area)
            .鉄筋系(true)
            .鉄骨系(true)
            .バストイレ別(true)
            .build()?;

        // ルーム一覧画面のURLを取得する
        let url = get_env_var("URL_SUUMO_KANTO_EKI_CHECK").unwrap();
        let res = self.client().get(url).query(&query).send().await?;
        let room_list_url = res.url().as_str();
        tracing::info!("{:#?}", room_list_url);
        Url::parse(room_list_url).context("Fail to Parse URL.")
    }

    // 最後のページ番号を確認し、各ページのURLを生成する
    #[tracing::instrument(skip_all, fields(url=url.as_str()), err(Debug))]
    async fn urls_of_room_list(&self, url: &Url) -> Result<Vec<Url>> {
        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;

        // ページネーションのパーツから最後のページ番号を取得する
        let html = res.text().await?;
        let max_page_number: usize = {
            let html = self.parse_html(&html);
            self.find_elements(&html, self.pagination_parts())
                .into_iter()
                .map(|element| self.inner_text(&element, ""))
                .map(|page_number| {
                    page_number
                        .parse::<usize>()
                        .expect("Fail to parse page number.")
                })
                .max()
                .expect("Fail to get max page number")
        };

        // 各ページ用のURLを生成する
        let urls: Result<Vec<Url>> = (1..=max_page_number)
            .into_iter()
            .map(|page| {
                Url::parse_with_params(url.as_str(), &[("page", page.to_string())])
                    .context("Fail to convert from page number to url.")
            })
            .collect();

        // 生成したURLを返す
        urls
    }

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    #[tracing::instrument(skip_all, fields(url=url.as_str()), err(Debug))]
    async fn residences_in_list_page(
        &self,
        url: &Url,
        area: TargetArea,
        station: &str,
    ) -> Result<Residences> {
        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;

        // 住居情報を取得する
        let url_domain = format!("{}://{}", url.scheme(), url.domain().unwrap());
        let text = res.text().await?;
        let residences = {
            let html = self.parse_html(&text);
            self.find_elements(&html, self.residence_root())
                .into_iter()
                .map(|element| {
                    let name =
                        self.find_inner_text_by_element(&element, self.residence_name(), ",");
                    let transfer =
                        self.find_inner_text_by_element(&element, self.residence_transfer(), "\n");
                    let rooms: Vec<Room> = self
                        .find_elements_by_element(&element, self.room_path())
                        .into_iter()
                        .map(|room| room.value().attr("href").expect("Fail to get room path."))
                        .map(|path| format!("{}{}", &url_domain, path))
                        .map(|url| url.into())
                        .collect::<Vec<Room>>();
                    Residence::new(
                        ResidenceHeader::new(name, transfer, area.clone(), station.to_string()),
                        rooms.into(),
                    )
                })
                .collect::<Vec<Residence>>()
        };
        let residences: Residences = residences.into();
        tracing::info!("{:#?}", residences);

        Ok(residences)
    }
}
