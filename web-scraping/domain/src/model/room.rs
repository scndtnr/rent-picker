use super::{RoomHeader, RoomRaw, RoomSanitize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room {
    header: RoomHeader,
    raw: Option<RoomRaw>,
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
    pub fn set_raw(&mut self, raw: RoomRaw) {
        self.raw = Some(raw)
    }
    pub fn set_sanitize(&mut self, sanitize: RoomSanitize) {
        self.sanitize = Some(sanitize)
    }

    pub fn header(&self) -> &RoomHeader {
        &self.header
    }

    pub fn raw(&self) -> &Option<RoomRaw> {
        &self.raw
    }

    pub fn sanitize(&self) -> &Option<RoomSanitize> {
        &self.sanitize
    }
}
