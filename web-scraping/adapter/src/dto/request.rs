use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RequestDto {
    pub dry_run: bool,
}
