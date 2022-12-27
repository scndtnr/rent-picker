use super::{selector, SearchQueryParams, SortType, Transfers};
use crate::repository::{crawler::HttpClient, HtmlParser};
use anyhow::{bail, Context, Result};
use domain::model::{Jst, RawRoom, RawRoomUpdateDate, RoomHeader, RoomHeaders, TargetArea};
use reqwest::Url;
use scraper::ElementRef;
use usecase::env::get_env_var;

#[async_trait::async_trait]
pub trait SuumoCrawler: HttpClient + HtmlParser {
    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(level = "trace", skip_all, err(Debug))]
    async fn health_check(&self) -> Result<()> {
        // suumo関東版のトップページを取得する
        let url = get_env_var("URL_SUUMO_KANTO_DOMAIN").unwrap();
        let res = self.client().get(&url).send().await?;

        // トップページのh1テキストを読む
        let text = res.text().await?;
        let top_kanto_title = self.innter_text_of_element(
            &text,
            selector::health_check::kanto_title().as_str(),
            ",",
        )?;
        tracing::info!("{}", top_kanto_title);

        // テキスト内容のチェック
        if top_kanto_title == "関東の住宅・不動産情報探し" {
            Ok(())
        } else {
            bail!("Unknown text: {}", text)
        }
    }

    /// 検索条件を指定して賃貸一覧ページのURLを取得する
    #[tracing::instrument(level = "trace", skip_all, fields(area=area.to_string(), station=station), err(Debug))]
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
    #[tracing::instrument(level = "trace", skip_all, fields(url=url.as_str()), err(Debug))]
    async fn urls_of_room_list(&self, url: &mut Url) -> Result<Vec<Url>> {
        // 賃貸一覧ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;
        self.sleep_by_secs(1).await;

        // ページネーションのパーツから最後のページ番号を取得する
        let html = res.text().await?;
        let max_page_number: usize = {
            let html = self.parse_html(&html);
            self.find_elements(&html, selector::room_header::pagination_parts().as_str())?
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

        // 1秒スリープする
        self.sleep_by_secs(1).await;

        // 住居情報を取得する
        let url_domain = format!("{}://{}", url.scheme(), url.domain().unwrap());
        let text = res.text().await?;
        let room_headers: Result<Vec<RoomHeader>> = {
            let html = self.parse_html(&text);
            let room_headers_vec = self
                .find_elements(&html, selector::room_header::residence_root().as_str())?
                .into_iter()
                .map(|element| {
                    match self.scrape_room_header(
                        element,
                        url_domain.clone(),
                        area,
                        station.to_string(),
                    ) {
                        Ok(headers) => Ok(headers),
                        Err(e) => bail!("{:#?}", e),
                    }
                })
                .collect::<Result<Vec<Vec<RoomHeader>>>>();
            Ok(room_headers_vec?.into_iter().flatten().collect())
        };

        Ok(room_headers?.into())
    }

    /// room_headerのスクレイピング部分
    fn scrape_room_header(
        &self,
        element: ElementRef,
        url_domain: String,
        area: &TargetArea,
        station: String,
    ) -> Result<Vec<RoomHeader>> {
        // 住居情報を取得する
        let building_name = self.find_inner_text_by_element(
            &element,
            selector::room_header::building_name().as_str(),
            ",",
        )?;
        let location = self.find_inner_text_by_element(
            &element,
            selector::room_header::location().as_str(),
            "\n",
        )?;
        let walk_to_station = self.find_inner_text_by_element(
            &element,
            selector::room_header::walk_to_station().as_str(),
            "\n",
        )?;
        let age_in_years = self.find_inner_text_by_element(
            &element,
            selector::room_header::age_in_years().as_str(),
            "\n",
        )?;
        let number_of_floors = self.find_inner_text_by_element(
            &element,
            selector::room_header::number_of_floors().as_str(),
            "\n",
        )?;
        let transfer_in_search_result = self.find_inner_text_by_element(
            &element,
            selector::room_header::transfer_in_search_result().as_str(),
            "\n",
        )?;

        //  各部屋のURLを取得し、Room構造体のVecに変換する
        let room_headers: Result<Vec<RoomHeader>> = self
            .find_elements_by_element(&element, selector::room_header::rooms().as_str())?
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
                )?;
                let room_rental_fee = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_rental_fee().as_str(),
                    "\n",
                )?;
                let room_management_fee = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_management_fee().as_str(),
                    "\n",
                )?;
                let room_security_deposit = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_security_deposit().as_str(),
                    "\n",
                )?;
                let room_key_money = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_key_money().as_str(),
                    "\n",
                )?;
                let room_floor_plan = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_floor_plan().as_str(),
                    "\n",
                )?;
                let room_private_area = self.find_inner_text_by_element(
                    &room,
                    selector::room_header::room_private_area().as_str(),
                    "^",
                )?;

                Ok(RoomHeader::new(
                    url,
                    building_name.clone(),
                    location.clone(),
                    walk_to_station.clone(),
                    age_in_years.clone(),
                    number_of_floors.clone(),
                    transfer_in_search_result.clone(),
                    area.clone(),
                    station.to_string(),
                    room_floor,
                    room_rental_fee,
                    room_management_fee,
                    room_security_deposit,
                    room_key_money,
                    room_floor_plan,
                    room_private_area,
                    Jst::now(),
                ))
            })
            .collect::<Result<Vec<RoomHeader>>>();
        room_headers
    }

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    #[tracing::instrument(level = "trace", skip_all, fields(url=url.as_str()) err(Debug))]
    async fn raw_room_in_detail_page(&self, url: &Url) -> Result<RawRoom> {
        // 賃貸詳細ページに遷移する
        let res = self.client().get(url.as_str()).send().await?;

        // 1秒スリープする
        self.sleep_by_secs(1).await;

        // リダイレクトURLを束縛しておく
        let redirect_url = res.url().clone();

        // 詳細情報をパースする
        let text = res.text().await?;
        let raw_room: RawRoom = {
            // Html構造体に変換する
            let html = self.parse_html(&text);

            // Sorryページかどうか判定する
            if let Ok(elem) = self.find_element(&html, selector::raw_room::sorry_message().as_str())
            {
                let msg = elem
                    .value()
                    .attr("alt")
                    .expect("Fail to unwrap sorry message");
                tracing::trace!("Sorry message found: {}", msg);
                return Ok(RawRoom::expired_new(url.as_str(), redirect_url.as_str()));
            };

            // libraryページかどうか判定する
            if let Ok(title) =
                self.find_inner_text(&html, selector::raw_room::library_page().as_str(), "")
            {
                tracing::trace!("Library page found: {}", title);
                return Ok(RawRoom::expired_new(url.as_str(), redirect_url.as_str()));
            };

            // その他スクレイピングできない状況か判定する
            if let Err(e) = self.find_element(&html, selector::raw_room::building_name().as_str()) {
                tracing::warn!("Buiding name is not found. Return empty struct: {}", e);
                return Ok(RawRoom::not_expired_new(
                    url.as_str(),
                    redirect_url.as_str(),
                ));
            };

            // 料金概要
            let building_name =
                self.find_inner_text(&html, selector::raw_room::building_name().as_str(), ",")?;

            // 料金概要
            let rental_fee =
                self.find_inner_text(&html, selector::raw_room::rental_fee().as_str(), "\n")?;
            let management_fee =
                self.find_inner_text(&html, selector::raw_room::management_fee().as_str(), "\n")?;
            let security_deposit =
                self.find_inner_text(&html, selector::raw_room::security_deposit().as_str(), "\n")?;
            let key_money =
                self.find_inner_text(&html, selector::raw_room::key_money().as_str(), "\n")?;
            let guarantee_deposit = self.find_inner_text(
                &html,
                selector::raw_room::guarantee_deposit().as_str(),
                "\n",
            )?;
            let key_money_amortization = self.find_inner_text(
                &html,
                selector::raw_room::key_money_amortization().as_str(),
                "\n",
            )?;

            // 建物概要
            let mut about_building_table = self
                .find_table_and_parse(&html, selector::raw_room::about_building_table().as_str())?;

            let location = about_building_table
                .remove("所在地")
                .unwrap_or_else(|| "N/A".to_string());
            let walk_to_station = about_building_table
                .remove("駅徒歩")
                .unwrap_or_else(|| "N/A".to_string());
            let floor_plan = about_building_table
                .remove("間取り")
                .unwrap_or_else(|| "N/A".to_string());
            let private_area = about_building_table
                .remove("専有面積")
                .unwrap_or_else(|| "N/A".to_string());
            let age_in_years = about_building_table
                .remove("築年数")
                .unwrap_or_else(|| "N/A".to_string());
            let floor = about_building_table
                .remove("階")
                .unwrap_or_else(|| "N/A".to_string());
            let facing_direction = about_building_table
                .remove("向き")
                .unwrap_or_else(|| "N/A".to_string());
            let building_type = about_building_table
                .remove("建物種別")
                .unwrap_or_else(|| "N/A".to_string());

            // 部屋の特徴・設備
            let features =
                self.find_inner_text(&html, selector::raw_room::features().as_str(), "\n")?;

            // 物件概要
            let mut about_room_table =
                self.find_table_and_parse(&html, selector::raw_room::about_room_table().as_str())?;

            let floor_plan_details = about_room_table
                .remove("間取り詳細")
                .unwrap_or_else(|| "N/A".to_string());
            let structure = about_room_table
                .remove("構造")
                .unwrap_or_else(|| "N/A".to_string());
            let number_of_floors = about_room_table
                .remove("階建")
                .unwrap_or_else(|| "N/A".to_string());
            let construction_date_yyyymm = about_room_table
                .remove("築年月")
                .unwrap_or_else(|| "N/A".to_string());
            let damage_insurance = about_room_table
                .remove("損保")
                .unwrap_or_else(|| "N/A".to_string());
            let parking = about_room_table
                .remove("駐車場")
                .unwrap_or_else(|| "N/A".to_string());
            let move_in = about_room_table
                .remove("入居")
                .unwrap_or_else(|| "N/A".to_string());
            let transaction_type = about_room_table
                .remove("取引態様")
                .unwrap_or_else(|| "N/A".to_string());
            let conditions = about_room_table
                .remove("条件")
                .unwrap_or_else(|| "N/A".to_string());
            let property_code = about_room_table
                .remove("取り扱い店舗物件コード")
                .unwrap_or_else(|| "N/A".to_string());
            let suumo_code = about_room_table
                .remove("SUUMO物件コード")
                .unwrap_or_else(|| "N/A".to_string());
            let contract_period = about_room_table
                .remove("契約期間")
                .unwrap_or_else(|| "N/A".to_string());
            let notes = about_room_table
                .remove("備考")
                .unwrap_or_else(|| "N/A".to_string());

            // （最終／次回）更新日時は別途変換する
            let update_date = RawRoomUpdateDate::new(
                about_room_table
                    .remove("情報更新日")
                    .unwrap_or_else(|| "N/A".to_string()),
                about_room_table
                    .remove("次回更新日")
                    .unwrap_or_else(|| "N/A".to_string()),
            );

            // 掲載終了フラグ
            // 特に問題なくスクレイピングできた場合は false
            let is_expired = false;

            // 念のため、取得していない項目を表示する。
            tracing::trace!(
                "about_building_table remaining: {:#?}",
                about_building_table
            );
            tracing::trace!("about_room_table remaining: {:#?}", about_room_table);

            RawRoom::new(
                url.to_string(),
                redirect_url.to_string(),
                Some(suumo_code),
                Some(building_name),
                Some(rental_fee),
                Some(management_fee),
                Some(security_deposit),
                Some(key_money),
                Some(guarantee_deposit),
                Some(key_money_amortization),
                Some(location),
                Some(walk_to_station),
                Some(floor_plan),
                Some(floor_plan_details),
                Some(private_area),
                Some(age_in_years),
                Some(construction_date_yyyymm),
                Some(floor),
                Some(number_of_floors),
                Some(facing_direction),
                Some(building_type),
                Some(features),
                Some(structure),
                Some(damage_insurance),
                Some(parking),
                Some(move_in),
                Some(transaction_type),
                Some(conditions),
                Some(property_code),
                Some(contract_period),
                Some(notes),
                Some(update_date.info_update_date()),
                Some(update_date.next_update_date()),
                Jst::now(),
                is_expired,
            )
        };

        Ok(raw_room)
    }
}
