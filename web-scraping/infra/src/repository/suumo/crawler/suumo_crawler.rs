use super::{selector, SearchQueryParams, SortType, Transfers};
use crate::repository::{crawler::HttpClient, HtmlParser};
use anyhow::{bail, Context, Result};
use domain::model::{Jst, RoomHeader, RoomHeaders, Rooms, TargetArea};
use reqwest::Url;
use usecase::env::get_env_var;

#[async_trait::async_trait]
pub trait SuumoCrawler: HttpClient + HtmlParser {
    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(skip_all, err(Debug))]
    async fn health_check(&self) -> Result<()> {
        // suumo関東版のトップページを取得する
        let url = get_env_var("URL_SUUMO_KANTO_DOMAIN").unwrap();
        let res = self.client().get(&url).send().await?;

        // トップページのh1テキストを読む
        let text = res.text().await?;
        let top_kanto_title =
            self.innter_text_of_element(&text, selector::health_check::kanto_title().as_str(), ",");
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
        Url::parse(room_list_url).context("Fail to Parse URL.")
    }

    // 最後のページ番号を確認し、各ページのURLを生成する
    #[tracing::instrument(skip_all, fields(url=url.as_str()), err(Debug))]
    async fn urls_of_room_list(&self, url: &mut Url) -> Result<Vec<Url>> {
        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;
        self.sleep_by_secs(1).await;

        // ページネーションのパーツから最後のページ番号を取得する
        let html = res.text().await?;
        let max_page_number: usize = {
            let html = self.parse_html(&html);
            self.find_elements(&html, selector::room_header::pagination_parts().as_str())
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
                Url::parse_with_params(
                    url.as_str(),
                    &[
                        ("page", page.to_string()),
                        ("po1", SortType::新着順.to_string()),
                    ],
                )
                .context("Fail to convert from page number to url.")
            })
            .collect();

        // 生成したURLを返す
        urls
    }

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    #[tracing::instrument(level = "trace", skip_all, fields(url=url.as_str()), err(Debug))]
    async fn room_headers_in_list_page(
        &self,
        url: &Url,
        area: &TargetArea,
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
            self.find_elements(&html, selector::room_header::residence_root().as_str())
                .into_iter()
                .flat_map(|element| {
                    // 住居情報を取得する
                    let residence_title = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_title().as_str(),
                        ",",
                    );
                    let residence_address = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_address().as_str(),
                        "\n",
                    );
                    let residence_nearest_station = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_nearest_station().as_str(),
                        "\n",
                    );
                    let residence_age = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_age().as_str(),
                        "\n",
                    );
                    let residence_floors = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_floors().as_str(),
                        "\n",
                    );
                    let residence_transfer = self.find_inner_text_by_element(
                        &element,
                        selector::room_header::residence_transfer().as_str(),
                        "\n",
                    );

                    //  各部屋のURLを取得し、Room構造体のVecに変換する
                    let room_headers: Vec<RoomHeader> = self
                        .find_elements_by_element(&element, selector::room_header::rooms().as_str())
                        .into_iter()
                        .map(|room| {
                            let url = format!(
                                "{}{}",
                                &url_domain,
                                self.find_element_by_element(
                                    &room,
                                    selector::room_header::room_path().as_str()
                                )
                                .unwrap()
                                .value()
                                .attr("href")
                                .expect("Fail to get room path.")
                            );
                            let room_floor = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_floor().as_str(),
                                "\n",
                            );
                            let room_rent_price = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_rent_price().as_str(),
                                "\n",
                            );
                            let room_condo_fee = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_condo_fee().as_str(),
                                "\n",
                            );
                            let room_deposit = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_deposit().as_str(),
                                "\n",
                            );
                            let room_key_money = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_key_money().as_str(),
                                "\n",
                            );
                            let room_layout = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_layout().as_str(),
                                "\n",
                            );
                            let room_exclusive_area = self.find_inner_text_by_element(
                                &room,
                                selector::room_header::room_exclusive_area().as_str(),
                                "^",
                            );

                            RoomHeader::new(
                                url,
                                residence_title.clone(),
                                residence_address.clone(),
                                residence_nearest_station.clone(),
                                residence_age.clone(),
                                residence_floors.clone(),
                                residence_transfer.clone(),
                                area.clone(),
                                station.to_string(),
                                room_floor,
                                room_rent_price,
                                room_condo_fee,
                                room_deposit,
                                room_key_money,
                                room_layout,
                                room_exclusive_area,
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
    #[allow(unused_variables)]
    #[tracing::instrument(skip_all, err(Debug))]
    async fn rooms_in_detail_page(&self, headers: RoomHeaders) -> Result<Rooms> {
        todo!();
    }
}
