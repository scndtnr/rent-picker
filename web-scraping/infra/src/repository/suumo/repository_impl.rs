use std::sync::Arc;

use anyhow::{Context, Result};
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, Rooms, TargetArea},
    repository::SuumoRepository,
};
use futures::{stream, StreamExt, TryStreamExt};
use reqwest::Url;

use crate::{
    progress_bar::{debug_progress, new_progress_bar},
    repository::ReqwestCrawler,
};
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
    #[tracing::instrument(level = "debug", skip_all, err(Debug))]
    async fn health_check(&self, crawler: &Self::Crawler) -> Result<()> {
        crawler.health_check().await
    }

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸一覧ページのURLを取得する
    #[tracing::instrument(level = "debug", skip(self, crawler), err(Debug))]
    async fn urls_of_list_page(
        &self,
        crawler: &Self::Crawler,
        area: &TargetArea,
        station: &str,
    ) -> Result<Vec<Url>> {
        // 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
        let mut url = crawler.url_of_room_list(area.clone(), station).await?;

        // 最後のページ番号を確認し、各ページのURLを生成する
        crawler.urls_of_room_list(&mut url).await
    }

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸の概要とURLを取得する
    #[tracing::instrument(level = "debug", skip_all, fields(urls_length=urls.len()) err(Debug))]
    async fn room_headers(
        &self,
        crawler: &Self::Crawler,
        urls: Vec<Url>,
        area: &TargetArea,
        station: &str,
    ) -> Result<RoomHeaders> {
        // 各賃貸一覧ページから住居情報や詳細ページへのURLを取得する
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let max_page = get_usize_of_env_var("MAX_PAGE");
        let urls = if urls.len() <= (max_page) {
            urls
        } else {
            urls[0..max_page].to_vec()
        };

        // プログレスバーの準備
        let pb_urls = new_progress_bar(urls.len() as u64).await;
        pb_urls.set_message("Web Scraping in list page...".to_string());

        let room_headers_vec: Vec<RoomHeaders> = stream::iter(urls)
            .map(|url| (url, area.clone(), station.to_string(), Arc::clone(&pb_urls)))
            .map(|(url, area, station, pb_urls)| async move {
                // 対象ページのスクレイピングをする
                let room_headers = crawler
                    .room_headers_in_list_page(&url, &area, &station)
                    .await
                    .context("Fail to parse room headers infomation.")?;
                // プログレスバーをインクリメントする
                pb_urls.inc(1);
                debug_progress(&pb_urls, "Web Scraping in list page...").await;

                anyhow::Ok(room_headers)
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await?;
        let room_headers: RoomHeaders = room_headers_vec
            .into_iter()
            .flat_map(|room_headers| room_headers.into_inner())
            .collect::<Vec<RoomHeader>>()
            .into();

        pb_urls.finish_with_message("Finish web scraping in list page.");

        Ok(room_headers)
    }

    /// RoomHeadersから、賃貸の詳細情報を取得する
    #[allow(unused_variables)]
    async fn rooms_by_room_headers(
        &self,
        crawler: &Self::Crawler,
        headers: &RoomHeaders,
    ) -> Result<Rooms> {
        todo!();
    }
}
