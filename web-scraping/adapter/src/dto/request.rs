use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequestDto {
    pub dry_run: bool,
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ScrapeSuumoRoomHeaderParamsDto {
    pub area: String,
    pub station: String,
    pub max_page: usize,
    pub save: bool,
    pub dry_run: bool,
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ScrapeSuumoRawRoomParamsDto {
    pub area: String,
    pub max_page: usize,
    pub save: bool,
    pub dry_run: bool,
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReadDbRequestDto {
    pub table_name: String,
    pub table_type: String,
}
