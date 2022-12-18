use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequestDto {
    pub dry_run: bool,
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SuumoRequestDto {
    pub area: String,
    pub station: String,
    pub save: bool,
    pub headers_from_database: bool,
    pub dry_run: bool,
}
