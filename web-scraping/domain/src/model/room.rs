use super::{RawRoom, RoomHeader, RoomSanitize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room {
    header: RoomHeader,
    raw: Option<RawRoom>,
    sanitize: Option<RoomSanitize>,
}

impl From<RoomHeader> for Room {
    fn from(header: RoomHeader) -> Self {
        Self::new(header)
    }
}

impl Room {
    pub fn new(header: RoomHeader) -> Self {
        Self {
            header,
            raw: None,
            sanitize: None,
        }
    }
    pub fn set_raw(&mut self, raw: RawRoom) {
        self.raw = Some(raw)
    }
    pub fn set_sanitize(&mut self, sanitize: RoomSanitize) {
        self.sanitize = Some(sanitize)
    }

    pub fn header(&self) -> &RoomHeader {
        &self.header
    }

    pub fn raw(&self) -> &Option<RawRoom> {
        &self.raw
    }

    pub fn sanitize(&self) -> &Option<RoomSanitize> {
        &self.sanitize
    }
}
