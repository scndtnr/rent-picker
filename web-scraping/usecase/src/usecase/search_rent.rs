use anyhow::Result;
use domain::{
    model::{AsVec, Residences, TargetArea},
    repository::{Repositories, SuumoRepository},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchRentUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
}

impl<R: Repositories> SearchRentUsecase<R> {
    pub fn new(suumo_repo: R::SuumoRepo) -> Self {
        Self { suumo_repo }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn search_rent_suumo(&self, area: TargetArea, station: &str) -> Result<Residences> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 指定した地域・通勤先の駅を元に賃貸情報を取得する
        let residences = self
            .suumo_repo
            .residences_by_area_and_station(&crawler, area, station)
            .await?;
        tracing::info!("{:#?}", residences.len());

        Ok(residences)
    }
}
