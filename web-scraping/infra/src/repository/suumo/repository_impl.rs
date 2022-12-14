use std::sync::Arc;

use anyhow::{bail, Context, Result};
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, Rooms, TargetArea},
    repository::SuumoRepository,
};
use futures::{stream, StreamExt, TryStreamExt};
use reqwest::Url;

use crate::repository::ReqwestCrawler;
use usecase::env::get_usize_of_env_var;

use super::SuumoCrawler;

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

    /// クローラ生成（Browser生成タイミングを制御するため）
    async fn new_crawler(&self) -> Arc<Self::Crawler> {
        Arc::new(Self::Crawler::new())
    }

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(skip_all, err(Debug))]
    async fn health_check(&self, crawler: &Self::Crawler) -> Result<()> {
        crawler.health_check().await
    }

    /// 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
    async fn url_of_room_list_by_area_and_station(
        &self,
        crawler: &Self::Crawler,
        area: &TargetArea,
        station: &str,
    ) -> Result<Url> {
        // 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
        crawler.url_of_room_list(area.clone(), station).await
    }

    /// 賃貸一覧ページの1ページ目のURLから、各ページのURLを生成する
    async fn urls_of_room_list_by_one_url(
        &self,
        crawler: &Self::Crawler,
        url: &mut Url,
    ) -> Result<Vec<Url>> {
        // 最後のページ番号を確認し、各ページのURLを生成する
        crawler.urls_of_room_list(url).await
    }

    /// 賃貸一覧ページのURLから、賃貸の概要とURLを取得する
    async fn room_headers_by_url(
        &self,
        crawler: Arc<Self::Crawler>,
        url: Url,
        area: TargetArea,
        station: String,
    ) -> Result<RoomHeaders> {
        crawler
            .room_headers_in_list_page(&url, &area, &station)
            .await
            .context("Fail to parse room headers infomation.")
    }

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸の概要とURLを取得する
    async fn room_headers_by_area_and_station(
        &self,
        crawler: &Self::Crawler,
        area: &TargetArea,
        station: &str,
    ) -> Result<RoomHeaders> {
        // 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
        let mut url = crawler.url_of_room_list(area.clone(), station).await?;

        // 最後のページ番号を確認し、各ページのURLを生成する
        let urls = crawler.urls_of_room_list(&mut url).await?;

        // 各賃貸一覧ページから住居情報や詳細ページへのURLを取得する
        tracing::info!("Urls length: {}", urls.len());
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let max_page = get_usize_of_env_var("MAX_PAGE");
        let urls = if urls.len() <= (max_page) {
            urls
        } else {
            urls[0..max_page].to_vec()
        };
        let room_headers_vec: Vec<RoomHeaders> = stream::iter(urls)
            .map(|url| (url, area.clone(), station.to_string()))
            .map(|(url, area, station)| async move {
                let room_headers = match crawler
                    .room_headers_in_list_page(&url, &area, &station)
                    .await
                {
                    Ok(room_headers) => room_headers,
                    Err(e) => bail!("Fail to parse room headers infomation. {:#?}", e),
                };
                Ok(room_headers)
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await?;
        let room_headers: RoomHeaders = room_headers_vec
            .into_iter()
            .flat_map(|room_headers| room_headers.into_inner())
            .collect::<Vec<RoomHeader>>()
            .into();
        Ok(room_headers)
    }

    /// RoomHeadersから、賃貸の詳細情報を取得する
    async fn rooms_by_room_headers(
        &self,
        crawler: &Self::Crawler,
        headers: &RoomHeaders,
    ) -> Result<Rooms> {
        todo!();
    }
}
