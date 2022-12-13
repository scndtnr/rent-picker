use super::builder::SearchQueryParamsBuilder;
use derive_new::new;
use serde_derive::{Deserialize, Serialize};

#[allow(clippy::too_many_arguments)]
#[derive(new, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SearchQueryParams {
    ar: String,
    bs: String,
    pc: String,
    #[serde(rename = "ekInput")]
    ek_input: String,
    #[serde(rename = "toEki")]
    to_eki: String,
    tj: String,
    nk: String,
    ta: String,
    cb: String,
    ct: String,
    et: String,
    cn: String,
    mb: String,
    mt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "kz")]
    kz1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "kz")]
    kz2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tc: Option<String>,
    shkr1: String,
    shkr2: String,
    shkr3: String,
    shkr4: String,
    fw2: String,
    po1: String,
}

impl Default for SearchQueryParams {
    fn default() -> Self {
        Self::builder().最寄り駅("八王子").build().unwrap()
    }
}

impl SearchQueryParams {
    pub fn builder() -> SearchQueryParamsBuilder {
        SearchQueryParamsBuilder::default()
    }
}
