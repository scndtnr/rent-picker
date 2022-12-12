use anyhow::{bail, Result};
use domain::{
    model::{AsVec, Residence, Residences, TargetArea},
    repository::SuumoRepository,
};
use futures::{stream, StreamExt, TryStreamExt};

use crate::{env::get_env_var, repository::ReqwestCrawler};

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
    async fn new_crawler(&self) -> Self::Crawler {
        Self::Crawler::new()
    }

    /// Suumoのヘルスチェック。トップページにログインできるかどうか。
    #[tracing::instrument(skip_all, err(Debug))]
    async fn health_check(&self, crawler: &Self::Crawler) -> Result<()> {
        crawler.health_check().await
    }

    /// 住居の属する地域や、通勤先の駅を指定して、賃貸情報を取得する
    #[tracing::instrument(skip_all, fields(area=area.to_string(), station=station) err(Debug))]
    async fn residences_by_area_and_station(
        &self,
        crawler: &Self::Crawler,
        area: TargetArea,
        station: &str,
    ) -> Result<Residences> {
        // 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
        let url = crawler.url_of_room_list(area, station).await?;

        // 最後のページ番号を確認し、各ページのURLを生成する
        let urls = crawler.urls_of_room_list(&url).await?;

        // 各賃貸一覧ページから住居情報や詳細ページへのURLを取得する
        tracing::info!("Urls length: {}", urls.len());
        let buffered_n = get_env_var("MAX_CONCURRENCY").unwrap().parse().unwrap();
        let max_page = get_env_var("MAX_PAGE").unwrap().parse().unwrap();
        let urls = if urls.len() <= (max_page) {
            urls
        } else {
            urls[0..max_page].to_vec()
        };
        let residences_vec: Vec<Residences> = stream::iter(urls)
            .map(|url| async move {
                let residences = match crawler.residences_in_list_page(&url).await {
                    Ok(residences) => residences,
                    Err(e) => bail!("Fail to parse residence infomation. {:#?}", e),
                };
                Ok(residences)
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await?;
        let resindences: Residences = residences_vec
            .into_iter()
            .flat_map(|residences| residences.into_inner())
            .collect::<Vec<Residence>>()
            .into();
        Ok(resindences)
    }
}
