use domain::model::TableType;

/// テーブル名を決定する
pub fn table_name(table: &TableType) -> &str {
    match table {
        TableType::Main => "raw_room",
        TableType::Load => "load_raw_room",
        TableType::Temp => "temp_raw_room",
    }
}

/// 最終更新が新しいか、あるいは掲載終了している
/// URLを重複なしで取得する select 文
pub fn select_latest_url(table: &TableType) -> String {
    let table = self::table_name(table);
    format!(
        "
        SELECT
            distinct url
        FROM
            {}
        WHERE
            next_update_date > CURRENT_TIMESTAMP
            OR is_expired == 1
        ",
        table
    )
}

/// PK毎にスクレイピング日時が最大のレコードを集約する select 文
/// is_expired は掲載終了ページを対象とするか否かのフラグ
pub fn select_group_by_pk(table: &TableType, is_expired: bool) -> String {
    let table = self::table_name(table);
    format!(
        "
        SELECT
            t.url
            ,t.redirect_url
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
            ,t.is_expired
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
            WHERE
                t.is_expired == {}
        ",
        table, table, is_expired as usize
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
                ,redirect_url
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
                ,is_expired
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
/// is_expired は掲載終了ページを対象とするか否かのフラグ
pub fn insert_from_other_table_group_by_pk(
    table: &TableType,
    other: &TableType,
    is_expired: bool,
) -> String {
    let table = self::table_name(table);
    let group_by_pk_from_other = self::select_group_by_pk(other, is_expired);
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
/// is_expired は掲載終了ページを対象とするか否かのフラグ
pub fn delete_where_group_by_pk_from_other_table(
    table: &TableType,
    other: &TableType,
    is_expired: bool,
) -> String {
    let table = self::table_name(table);
    let group_by_pk_from_other = self::select_group_by_pk(other, is_expired);
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

/// 他のテーブルにのみ存在する、
/// is_expired で選択されたレコードを追加する insert文
pub fn insert_from_other_table_by_is_expired(
    table: &TableType,
    other: &TableType,
    is_expired: bool,
) -> String {
    let table = self::table_name(table);
    let other = self::table_name(other);
    format!(
        "
            INSERT INTO {}
            SELECT
                other.*
            FROM
                {} other
                left outer join {} t ON other.url = t.url
            WHERE
                other.is_expired = {}
                AND t.url is null
        ",
        table, other, table, is_expired as usize
    )
}

/// raw_room 系テーブルからPKに合致したレコードを削除する delete文
/// PKは他の raw_room 系テーブルから引っ張ってくる
/// is_expired は掲載終了ページを対象とするか否かのフラグ
pub fn update_is_expired_column_by_other_table(
    table: &TableType,
    other: &TableType,
    is_expired: bool,
) -> String {
    let table = self::table_name(table);
    let group_by_pk_from_other = self::select_group_by_pk(other, is_expired);
    format!(
        "
            UPDATE {}
            SET 
                is_expired = {}
            WHERE
                url in (
                    SELECT
                        other.url
                    FROM
                        ({}) other
                )
        ",
        table, is_expired as usize, group_by_pk_from_other
    )
}
