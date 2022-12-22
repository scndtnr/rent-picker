use domain::model::TableType;

/// テーブル名を決定する
pub fn table_name(table: &TableType) -> &str {
    match table {
        TableType::Main => "raw_room",
        TableType::Load => "load_raw_room",
        TableType::Temp => "temp_raw_room",
    }
}

/// 最新のURLを重複なしで取得する select 文
pub fn select_updated_url(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        SELECT
            distinct url
        FROM
            {}
        WHERE
            next_update_date > CURRENT_TIMESTAMP
        ",
        table
    )
}
