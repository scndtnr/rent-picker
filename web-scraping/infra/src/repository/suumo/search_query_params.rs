use anyhow::{bail, Context, Result};
use domain::model::TargetArea;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Transfers {
    こだわらない,
    乗り換えなし,
    乗り換え1回以内,
    乗り換え2回以内,
}

impl std::fmt::Display for Transfers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transfers::こだわらない => write!(f, "-1"),
            Transfers::乗り換えなし => write!(f, "0"),
            Transfers::乗り換え1回以内 => write!(f, "1"),
            Transfers::乗り換え2回以内 => write!(f, "2"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SearchQueryParamsBuilder {
    hidden_検索フォーム_ar: String,
    hidden_検索フォーム_bs: String,
    hidden_検索フォーム_pc: String,
    hidden_最寄り駅入力: String,
    最寄り駅: Option<String>,
    電車での所要時間: String,
    乗り換え回数: Transfers,
    住みたいエリア: TargetArea,
    賃料下限: Option<String>,
    賃料上限: Option<String>,
    駅徒歩: String,
    築後年数: Option<String>,
    専有面積下限: Option<String>,
    専有面積上限: Option<String>,
    鉄筋系: bool,
    鉄骨系: bool,
    バストイレ別: bool,
    hidden_周辺環境1: String,
    hidden_周辺環境2: String,
    hidden_周辺環境3: String,
    hidden_周辺環境4: String,
    hidden_テキストボックス: String,
}

impl Default for SearchQueryParamsBuilder {
    fn default() -> Self {
        Self {
            hidden_検索フォーム_ar: "030".to_string(),
            hidden_検索フォーム_bs: "040".to_string(),
            hidden_検索フォーム_pc: "30".to_string(),
            hidden_最寄り駅入力: "".to_string(),
            最寄り駅: None,
            電車での所要時間: "10".to_string(),
            乗り換え回数: Transfers::こだわらない,
            住みたいエリア: TargetArea::Tokyo,
            賃料下限: Some("0.0".to_string()),
            賃料上限: Some("9999999".to_string()),
            駅徒歩: "9999999".to_string(),
            築後年数: Some("9999999".to_string()),
            専有面積下限: Some("0".to_string()),
            専有面積上限: Some("9999999".to_string()),
            鉄筋系: true,
            鉄骨系: true,
            バストイレ別: true,
            hidden_周辺環境1: "03".to_string(),
            hidden_周辺環境2: "03".to_string(),
            hidden_周辺環境3: "03".to_string(),
            hidden_周辺環境4: "03".to_string(),
            hidden_テキストボックス: "".to_string(),
        }
    }
}

impl SearchQueryParamsBuilder {
    pub fn 最寄り駅(mut self, eki: impl Into<String>) -> Self {
        self.最寄り駅 = Some(eki.into());
        self
    }

    pub fn 住みたいエリア(mut self, area: TargetArea) -> Self {
        self.住みたいエリア = area;
        self
    }

    /// range = ["10", "20", "30", "40", "50", "60", "70", "80", "90"];
    pub fn 電車での所要時間(mut self, minutes: &str) -> Result<Self> {
        let range = ["10", "20", "30", "40", "50", "60", "70", "80", "90"];
        if range.contains(&minutes) {
            self.電車での所要時間 = minutes.to_string();
            Ok(self)
        } else {
            bail!("Invalid mitutes. minutes: {} range: {:#?} ", minutes, range)
        }
    }

    pub fn 乗り換え回数(mut self, transfers: Transfers) -> Self {
        self.乗り換え回数 = transfers;
        self
    }

    /// range = ["1", "5", "7", "10", "15", "20", "9999999"];
    pub fn 駅徒歩(mut self, minutes: &str) -> Result<Self> {
        let range = ["1", "5", "7", "10", "15", "20", "9999999"];
        if range.contains(&minutes) {
            self.駅徒歩 = minutes.to_string();
            Ok(self)
        } else {
            bail!("Invalid mitutes. minutes: {} range: {:#?} ", minutes, range)
        }
    }

    /// default: true
    pub fn 鉄筋系(mut self, check: bool) -> Self {
        self.鉄筋系 = check;
        self
    }

    /// default: true
    pub fn 鉄骨系(mut self, check: bool) -> Self {
        self.鉄骨系 = check;
        self
    }

    /// default: true
    pub fn バストイレ別(mut self, check: bool) -> Self {
        self.バストイレ別 = check;
        self
    }

    pub fn build(self) -> Result<SearchQueryParams> {
        Ok(SearchQueryParams {
            ar: self.hidden_検索フォーム_ar,
            bs: self.hidden_検索フォーム_bs,
            pc: self.hidden_検索フォーム_pc,
            ek_input: self.hidden_最寄り駅入力,
            to_eki: self
                .最寄り駅
                .with_context(|| format!("Field `{}` is mandatory.", "'to_eki' or '最寄り駅'"))?,
            tj: self.電車での所要時間,
            nk: self.乗り換え回数.to_string(),
            ta: match self.住みたいエリア {
                TargetArea::Tokyo => "13".to_string(),
                TargetArea::Kanagawa => "14".to_string(),
                TargetArea::Saitama => "11".to_string(),
                TargetArea::Chiba => "12".to_string(),
            },
            cb: self.賃料下限.unwrap(),
            ct: self.賃料上限.unwrap(),
            et: self.駅徒歩,
            cn: self.築後年数.unwrap(),
            mb: self.専有面積下限.unwrap(),
            mt: self.専有面積上限.unwrap(),
            kz1: if self.鉄筋系 {
                Some("1".to_string())
            } else {
                None
            },
            kz2: if self.鉄骨系 {
                Some("2".to_string())
            } else {
                None
            },
            tc: if self.バストイレ別 {
                Some("0400301".to_string())
            } else {
                None
            },
            shkr1: self.hidden_周辺環境1,
            shkr2: self.hidden_周辺環境2,
            shkr3: self.hidden_周辺環境3,
            shkr4: self.hidden_周辺環境4,
            fw2: self.hidden_テキストボックス,
        })
    }
}
