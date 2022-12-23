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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ScrapeSuumoRawRoomParamsDto {
    pub area: String,
    pub max_page: usize,
    pub chunk_size: usize,
    pub save: bool,
    pub dry_run: bool,
}

impl ScrapeSuumoRawRoomParamsDto {
    pub fn new(
        area: String,
        max_page: usize,
        chunk_size: usize,
        save: bool,
        dry_run: bool,
    ) -> Self {
        if max_page < chunk_size {
            tracing::error!(
                "Invalid argument. 'max_page' must be larger than 'chunk_size'. Current argument: {{ max_page: {}, chunk_size: {} }}",
                max_page,
                chunk_size
            );
            panic!();
        }
        Self {
            area,
            max_page,
            chunk_size,
            save,
            dry_run,
        }
    }
}

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ReadDbRequestDto {
    pub table_name: String,
    pub table_type: String,
}
