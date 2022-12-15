use anyhow::bail;

/// データベースのテーブルを用途毎に分類するenum
/// 本テーブル、累積用作業テーブル、一時的な作業テーブルの3種類
#[derive(Debug, Clone)]
pub enum TableType {
    /// 本テーブル
    Main,
    /// 累積用の作業テーブル
    Load,
    /// 一時的な作業テーブル
    Temp,
}

impl TryFrom<String> for TableType {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_uppercase() {
            table if table == "MAIN" => Ok(TableType::Main),
            table if table == "LOAD" => Ok(TableType::Load),
            table if table == "TEMP" => Ok(TableType::Temp),
            table => bail!("Unknown table: {}", table),
        }
    }
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableType::Main => write!(f, "Main"),
            TableType::Load => write!(f, "Load"),
            TableType::Temp => write!(f, "Temp"),
        }
    }
}
