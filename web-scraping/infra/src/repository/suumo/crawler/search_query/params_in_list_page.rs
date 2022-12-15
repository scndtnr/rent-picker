use serde_derive::{Deserialize, Serialize};

/// query-params の key は "po1"
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SortType {
    おすすめ順,
    賃料と管理費の合計が安い順,
    賃料と管理費の合計が高い順,
    新着順,
    築年数が新しい順,
    専有面積が広い順,
    住所別,
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortType::おすすめ順 => write!(f, "25"),
            SortType::賃料と管理費の合計が安い順 => write!(f, "12"),
            SortType::賃料と管理費の合計が高い順 => write!(f, "15"),
            SortType::新着順 => write!(f, "09"),
            SortType::築年数が新しい順 => write!(f, "04"),
            SortType::専有面積が広い順 => write!(f, "16"),
            SortType::住所別 => write!(f, "17"),
        }
    }
}
