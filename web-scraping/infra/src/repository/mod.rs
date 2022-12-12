mod crawler;
mod suumo;

pub(self) use crawler::{HtmlParser, ReqwestCrawler};
use domain::repository::Repositories;
use suumo::SuumoRepositoryImpl;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RepositoryImpls {
    suumo_repository: SuumoRepositoryImpl,
}

impl Repositories for RepositoryImpls {
    type SuumoRepo = SuumoRepositoryImpl;

    fn suumo_repository(&self) -> &Self::SuumoRepo {
        &self.suumo_repository
    }
}

impl RepositoryImpls {
    pub fn new() -> Self {
        let suumo_repository = SuumoRepositoryImpl::new();
        Self { suumo_repository }
    }
}

impl Default for RepositoryImpls {
    fn default() -> Self {
        Self::new()
    }
}
