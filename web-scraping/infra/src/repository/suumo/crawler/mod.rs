mod search_query;
mod selector;
mod suumo_crawler;

pub(self) use search_query::{SearchQueryParams, SortType, Transfers};
pub(self) use selector::SuumoSelector;
pub(super) use suumo_crawler::SuumoCrawler;

use crate::repository::ReqwestCrawler;

impl SuumoSelector for ReqwestCrawler {}
impl SuumoCrawler for ReqwestCrawler {}
