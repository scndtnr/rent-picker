use super::{RawRoomRepository, RoomHeaderRepository, SuumoRepository};

pub trait Repositories {
    type SuumoRepo: SuumoRepository;
    type RoomHeaderRepo: RoomHeaderRepository;
    type RawRoomRepo: RawRoomRepository;

    fn suumo_repository(&self) -> &Self::SuumoRepo;
    fn room_header_repository(&self) -> &Self::RoomHeaderRepo;
    fn raw_room_repository(&self) -> &Self::RawRoomRepo;
}
