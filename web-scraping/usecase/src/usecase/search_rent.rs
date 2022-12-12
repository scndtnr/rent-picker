use anyhow::Result;
use domain::{
    model::{Residences, TargetArea},
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
        let selector = self.suumo_repo.new_selector().await;

        // 検索条件を指定して遷移する
        let url = self
            .suumo_repo
            .url_of_room_list(&crawler, area, station)
            .await?;
        let _titles = self
            .suumo_repo
            .residences_in_list_page(&crawler, &selector, url)
            .await?;

        todo!()
    }
}
