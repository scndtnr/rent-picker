use anyhow::bail;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TargetArea {
    Tokyo,
    Kanagawa,
    Saitama,
    Chiba,
}

impl TryFrom<String> for TargetArea {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_uppercase() {
            area if area == "TOKYO" => Ok(TargetArea::Tokyo),
            area if area == "KANAGAWA" => Ok(TargetArea::Kanagawa),
            area if area == "SAITAMA" => Ok(TargetArea::Saitama),
            area if area == "CHIBA" => Ok(TargetArea::Chiba),
            area => bail!("Unknown area: {}", area),
        }
    }
}

impl std::fmt::Display for TargetArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetArea::Tokyo => write!(f, "Tokyo"),
            TargetArea::Kanagawa => write!(f, "Kanagawa"),
            TargetArea::Saitama => write!(f, "Saitama"),
            TargetArea::Chiba => write!(f, "Chiba"),
        }
    }
}
