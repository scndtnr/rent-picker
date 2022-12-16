mod search_query;
pub(self) mod selector;
mod suumo_crawler;

pub(self) use search_query::{SearchQueryParams, SortType, Transfers};
pub(super) use suumo_crawler::SuumoCrawler;

use crate::repository::ReqwestCrawler;

impl SuumoCrawler for ReqwestCrawler {}
