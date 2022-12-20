mod crawler;
mod sqlite;
mod suumo;

pub(self) use crawler::{HtmlParser, ReqwestCrawler};
use domain::{
    model::{RawRoom, RoomHeader},
    repository::Repositories,
};
use suumo::SuumoRepositoryImpl;

use crate::persistence::sqlite::SqliteDb;

use self::sqlite::SqliteRepositoryImpl;

#[derive(Debug, Clone)]
pub struct RepositoryImpls {
    suumo_repository: SuumoRepositoryImpl,
    room_header_repository: SqliteRepositoryImpl<RoomHeader>,
    raw_room_repository: SqliteRepositoryImpl<RawRoom>,
}

impl Repositories for RepositoryImpls {
    type SuumoRepo = SuumoRepositoryImpl;
    type RoomHeaderRepo = SqliteRepositoryImpl<RoomHeader>;
    type RawRoomRepo = SqliteRepositoryImpl<RawRoom>;

    fn suumo_repository(&self) -> &Self::SuumoRepo {
        &self.suumo_repository
    }
    fn room_header_repository(&self) -> &Self::RoomHeaderRepo {
        &self.room_header_repository
    }
    fn raw_room_repository(&self) -> &Self::RawRoomRepo {
        &self.raw_room_repository
    }
}

impl RepositoryImpls {
    pub fn new(db: SqliteDb) -> Self {
        let suumo_repository = SuumoRepositoryImpl::new();
        let room_header_repository = SqliteRepositoryImpl::new(db.clone());
        let raw_room_repository = SqliteRepositoryImpl::new(db);
        Self {
            suumo_repository,
            room_header_repository,
            raw_room_repository,
        }
    }
}
