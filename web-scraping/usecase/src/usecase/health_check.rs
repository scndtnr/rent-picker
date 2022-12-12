use anyhow::Result;
use domain::repository::{Repositories, SuumoRepository};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HealthCheckUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
}

impl<R: Repositories> HealthCheckUsecase<R> {
    pub fn new(suumo_repo: R::SuumoRepo) -> Self {
        Self { suumo_repo }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn health_check_suumo(&self) -> Result<()> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // ヘルスチェック結果を返す
        self.suumo_repo.health_check(&crawler).await
    }
}
