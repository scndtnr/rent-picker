use serde_derive::{Deserialize, Serialize};

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
