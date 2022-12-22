use domain::model::{TableType, TargetArea};

/// テーブル名を決定する
pub fn table_name(table: &TableType) -> &str {
    match table {
        TableType::Main => "room_header",
        TableType::Load => "load_room_header",
        TableType::Temp => "temp_room_header",
    }
}

/// raw_roomテーブルに存在しない、あるいは最終更新が古いURLで、
/// かつ指定されたエリアと合致するレコードを取得する select文
pub fn select_unscraped_raw_room_urls_filtered_by_area(
    table: &TableType,
    area: &TargetArea,
) -> String {
    let header_table = self::table_name(table);
    let latest_urls_in_raw_room_table = super::raw_room::select_latest_url(table);
    format!(
        "
        SELECT
            header.url
        FROM
            {} header
            LEFT OUTER JOIN ({}) raw on header.url = raw.url
        WHERE
            area_of_search_condition = '{}'
            and raw.url is null
        ",
        header_table, latest_urls_in_raw_room_table, area
    )
}

/// PK毎にスクレイピング日時が最大のレコードを集約する select 文
pub fn select_group_by_pk(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        SELECT
            t.url
            ,t.building_name
            ,t.location
            ,t.walk_to_station
            ,t.age_in_years
            ,t.number_of_floors
            ,t.transfer_in_search_result
            ,t.area_of_search_condition
            ,t.commute_station_of_search_condition
            ,t.floor
            ,t.rental_fee
            ,t.management_fee
            ,t.security_deposit
            ,t.key_money
            ,t.floor_plan
            ,t.private_area
            ,t.scraping_date
        FROM
            {} t
            JOIN (
                SELECT
                    url,
                    max(scraping_date) max_scraping_date
                FROM
                    {}
                GROUP BY
                    url
            ) g
                ON t.url = g.url
                AND t.scraping_date = g.max_scraping_date
        ",
        table, table
    )
}

/// room_header 系テーブルへの insert文
pub fn insert_all_columns(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        INSERT INTO {}
            (
                url
                ,building_name
                ,location
                ,walk_to_station
                ,age_in_years
                ,number_of_floors
                ,transfer_in_search_result
                ,area_of_search_condition
                ,commute_station_of_search_condition
                ,floor
                ,rental_fee
                ,management_fee
                ,security_deposit
                ,key_money
                ,floor_plan
                ,private_area
                ,scraping_date
            )
    ",
        table
    )
}

/// room_header 系のテーブルからテーブルへの全件 insert文
pub fn insert_from_other_table_all(table: &TableType, other: &TableType) -> String {
    let table = self::table_name(table);
    let other = self::table_name(other);
    format!(
        "
                INSERT INTO {}
                SELECT * FROM {}
            ",
        table, other
    )
}

/// room_header 系のテーブルからPKで集約したデータを
/// 同じく room_header 系のテーブルへ入れ込む insert文
pub fn insert_from_other_table_group_by_pk(table: &TableType, other: &TableType) -> String {
    let group_by_pk_from_other = self::select_group_by_pk(other);
    let table = self::table_name(table);
    format!(
        "
                INSERT INTO {}
                SELECT other.* FROM ({}) other
            ",
        table, group_by_pk_from_other
    )
}

/// room_header 系テーブルのデータを全削除する delete文
pub fn delete_all(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
            DELETE FROM {}
            ",
        table
    )
}

/// room_header 系テーブルからPKに合致したレコードを削除する delete文
/// PKは他の room_header 系テーブルから引っ張ってくる
pub fn delete_where_group_by_pk_from_other_table(table: &TableType, other: &TableType) -> String {
    let table = self::table_name(table);
    let group_by_pk_from_other = self::select_group_by_pk(other);
    format!(
        "
                DELETE FROM {}
                WHERE
                    url in (
                        SELECT
                            other.url
                        FROM
                            ({}) other
                    )
                ",
        table, group_by_pk_from_other
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 詳細データ未取得のヘッダ情報を呼ぶクエリ() {
        let table = TableType::Main;
        let area = TargetArea::Tokyo;
        let sql = select_unscraped_raw_room_urls_filtered_by_area(&table, &area);
        // println!("{}", sql);
        let extended_sql = "
        SELECT
            header.url
        FROM
            room_header header
            LEFT OUTER JOIN (
        SELECT
            distinct url
        FROM
            raw_room
        WHERE
            next_update_date > CURRENT_TIMESTAMP
            OR is_expired == 1
        ) raw on header.url = raw.url
        WHERE
            area_of_search_condition = 'Tokyo'
            and raw.url is null
        ";
        assert_eq!(&sql, extended_sql);
    }
}
