use super::{RoomRaw, RoomSanitize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room {
    url: String,
    raw: Option<RoomRaw>,
    sanitize: Option<RoomSanitize>,
}

impl From<String> for Room {
    fn from(url: String) -> Self {
        Self::new(url)
    }
}

impl Room {
    pub fn new(url: String) -> Self {
        Self {
            url,
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

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn raw(&self) -> &Option<RoomRaw> {
        &self.raw
    }

    pub fn sanitize(&self) -> &Option<RoomSanitize> {
        &self.sanitize
    }
}
