use super::{RoomHeaderRepository, SuumoRepository};

pub trait Repositories {
    type SuumoRepo: SuumoRepository;
    type RoomHeaderRepo: RoomHeaderRepository;

    fn suumo_repository(&self) -> &Self::SuumoRepo;
    fn room_header_repository(&self) -> &Self::RoomHeaderRepo;
}
