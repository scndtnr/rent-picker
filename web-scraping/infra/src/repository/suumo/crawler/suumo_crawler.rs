use super::{selector::SuumoSelector, SearchQueryParams, SortType, Transfers};
use crate::{
    env::get_env_var,
    repository::{crawler::HttpClient, HtmlParser},
};
use anyhow::{bail, Context, Result};
use domain::model::{Jst, RoomHeader, RoomHeaders, Rooms, TargetArea};
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
            .並び替え(SortType::新着順)
            .build()?;

        // ルーム一覧画面のURLを取得する
        let url = get_env_var("URL_SUUMO_KANTO_EKI_CHECK").unwrap();
        let res = self.client().get(url).query(&query).send().await?;
        let room_list_url = res.url().as_str();
        Url::parse(room_list_url).context("Fail to Parse URL.")
    }

    // 最後のページ番号を確認し、各ページのURLを生成する
    #[tracing::instrument(skip_all, fields(url=url.as_str()), err(Debug))]
    async fn urls_of_room_list(&self, url: &mut Url) -> Result<Vec<Url>> {
        // scheme を https から http に変更する
        // [Error: IncompleteMessage: connection closed before message completed · Issue #2136 · hyperium/hyper](https://github.com/hyperium/hyper/issues/2136)
        url.set_scheme("http")
            .expect("Fail to change scheme from 'https' to 'http'");

        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;
        self.sleep_by_secs(1).await;

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
    async fn room_headers_in_list_page(
        &self,
        url: &Url,
        area: TargetArea,
        station: &str,
    ) -> Result<RoomHeaders> {
        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;
        self.sleep_by_secs(1).await;

        // 住居情報を取得する
        let url_domain = format!("{}://{}", url.scheme(), url.domain().unwrap());
        let text = res.text().await?;
        let room_headers: RoomHeaders = {
            let html = self.parse_html(&text);
            self.find_elements(&html, self.residence_root())
                .into_iter()
                .flat_map(|element| {
                    // 住居情報を取得する
                    let residence_title =
                        self.find_inner_text_by_element(&element, self.residence_name(), ",");
                    let residence_transfer =
                        self.find_inner_text_by_element(&element, self.residence_transfer(), "\n");

                    //  各部屋のURLを取得し、Room構造体のVecに変換する
                    let room_headers: Vec<RoomHeader> = self
                        .find_elements_by_element(&element, self.room_path())
                        .into_iter()
                        .map(|room| room.value().attr("href").expect("Fail to get room path."))
                        .map(|path| format!("{}{}", &url_domain, path))
                        .into_iter()
                        .map(|url| {
                            RoomHeader::new(
                                url,
                                residence_title.clone(),
                                residence_transfer.clone(),
                                area.clone(),
                                station.to_string(),
                                Jst::now(),
                            )
                        })
                        .collect();
                    room_headers
                })
                .collect::<Vec<RoomHeader>>()
                .into()
        };

        Ok(room_headers)
    }

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    #[tracing::instrument(skip_all, err(Debug))]
    async fn rooms_in_detail_page(&self, headers: RoomHeaders) -> Result<Rooms> {
        todo!();
    }
}
