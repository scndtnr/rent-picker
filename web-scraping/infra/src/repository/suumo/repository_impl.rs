use anyhow::{bail, Context, Result};
use domain::{
    model::{Residences, TargetArea},
    repository::SuumoRepository,
};
use futures::{stream, StreamExt, TryStreamExt};
use url::Url;

use crate::{
    env::get_env_var,
    repository::{
        suumo::{SearchQueryParams, Transfers},
        ReqwestCrawler,
    },
};

use super::SuumoSelector;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoRepositoryImpl;

impl SuumoRepositoryImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SuumoRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SuumoRepository for SuumoRepositoryImpl {
    type Crawler = ReqwestCrawler;
    type Selector = SuumoSelector;

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Self::Crawler {
        Self::Crawler::new()
    }

    /// セレクタ生成
    async fn new_selector(&self) -> Self::Selector {
        Self::Selector::default()
    }

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(skip_all, err(Debug))]
    async fn health_check(&self, crawler: &Self::Crawler, selector: &Self::Selector) -> Result<()> {
        // suumo関東版のトップページを取得する
        let url = get_env_var("URL_SUUMO_KANTO_DOMAIN").unwrap();
        let res = crawler.client().get(&url).send().await?;

        // トップページのh1テキストを読む
        let text = res.text().await?;
        let top_kanto_title =
            ReqwestCrawler::innter_text_of_element(&text, &selector.top.kanto_title, ",");
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
    async fn url_of_room_list(
        &self,
        crawler: &Self::Crawler,
        area: TargetArea,
        station: &str,
    ) -> Result<Url> {
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
        let res = crawler.client().get(url).query(&query).send().await?;
        let room_list_url = res.url().as_str();
        tracing::info!("{:#?}", room_list_url);
        Url::parse(room_list_url).context("Fail to Parse URL.")
    }

    /// 賃貸一覧ページから賃貸情報や詳細ページのURLを取得する
    async fn residences_in_list_page(
        &self,
        crawler: &Self::Crawler,
        selector: &Self::Selector,
        url: Url,
    ) -> Result<Residences> {
        // 賃貸一覧ページに遷移する
        let res = crawler.client().get(url.as_str()).send().await?;

        // 各住居名を取得する
        let text = res.text().await?;
        let titles = {
            let html = ReqwestCrawler::parse_html(&text);
            ReqwestCrawler::find_elements(&html, "div.cassetteitem_content-title")
                .into_iter()
                .map(|elem| ReqwestCrawler::inner_text(&elem, ","))
                .collect::<Vec<String>>()
        };
        tracing::info!("{:#?}", titles);

        todo!()
    }
}
