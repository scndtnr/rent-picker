use derive_new::new;

use super::{RoomRaw, RoomSanitize};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room {
    raw: RoomRaw,
    sanitize: RoomSanitize,
}
