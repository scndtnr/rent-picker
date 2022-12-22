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

/// PK毎にスクレイピング日時が最大のレコードを集約する select 文
pub fn select_group_by_pk(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        SELECT
            t.url
            ,t.suumo_code
            ,t.building_name
            ,t.rental_fee
            ,t.management_fee
            ,t.security_deposit
            ,t.key_money
            ,t.guarantee_deposit
            ,t.key_money_amortization
            ,t.location
            ,t.walk_to_station
            ,t.floor_plan
            ,t.floor_plan_details
            ,t.private_area
            ,t.age_in_years
            ,t.construction_date_yyyymm
            ,t.floor
            ,t.number_of_floors
            ,t.facing_direction
            ,t.building_type
            ,t.features
            ,t.structure
            ,t.damage_insurance
            ,t.parking
            ,t.move_in
            ,t.transaction_type
            ,t.conditions
            ,t.property_code
            ,t.contract_period
            ,t.notes
            ,t.info_update_date
            ,t.next_update_date
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

/// raw_room 系テーブルへの insert文
pub fn insert_all_columns(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        INSERT INTO {}
            (
                url
                ,suumo_code
                ,building_name
                ,rental_fee
                ,management_fee
                ,security_deposit
                ,key_money
                ,guarantee_deposit
                ,key_money_amortization
                ,location
                ,walk_to_station
                ,floor_plan
                ,floor_plan_details
                ,private_area
                ,age_in_years
                ,construction_date_yyyymm
                ,floor
                ,number_of_floors
                ,facing_direction
                ,building_type
                ,features
                ,structure
                ,damage_insurance
                ,parking
                ,move_in
                ,transaction_type
                ,conditions
                ,property_code
                ,contract_period
                ,notes
                ,info_update_date
                ,next_update_date
                ,scraping_date
            )
        ",
        table
    )
}

/// raw_room 系のテーブルからテーブルへの全件 insert文
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

/// raw_room 系のテーブルからPKで集約したデータを
/// 同じく raw_room 系のテーブルへ入れ込む insert文
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

/// raw_room 系テーブルのデータを全削除する delete文
pub fn delete_all(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
            DELETE FROM {}
            ",
        table
    )
}

/// raw_room 系テーブルからPKに合致したレコードを削除する delete文
/// PKは他の raw_room 系テーブルから引っ張ってくる
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
