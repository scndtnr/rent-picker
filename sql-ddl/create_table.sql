-- room_header definition

/* --------- room_header ------------- */

-- DROP TABLE room_header;
CREATE TABLE room_header (
    -- 詳細ページのURL
    url VARCHAR NOT NULL,
    -- 建物名
    building_name VARCHAR,
    -- 所在地
    location VARCHAR,
    -- 駅徒歩
    walk_to_station VARCHAR,
    -- 築年数
    age_in_years VARCHAR,
    -- 階建
    number_of_floors VARCHAR,
    -- 乗換回数（検索結果）
    transfer_in_search_result VARCHAR,
    -- 都道府県エリア（検索条件）
    area_of_search_condition VARCHAR,
    -- 通勤先の最寄り駅（検索条件）
    commute_station_of_search_condition VARCHAR,
    -- 階
    floor VARCHAR,
    -- 家賃
    rental_fee VARCHAR,
    -- 管理費共益費
    management_fee VARCHAR,
    -- 敷金
    security_deposit VARCHAR,
    -- 礼金
    key_money VARCHAR,
    -- 間取り
    floor_plan VARCHAR,
    -- 専有面積
    private_area VARCHAR,
    -- スクレイピングした日時
    scraping_date  DATETIME,
    ---------------------------------
    -- PK制約
    PRIMARY KEY (url)
);

-- DROP TABLE load_room_header;
CREATE TABLE load_room_header (
    url VARCHAR NOT NULL,
    building_name VARCHAR,
    location VARCHAR,
    walk_to_station VARCHAR,
    age_in_years VARCHAR,
    number_of_floors VARCHAR,
    transfer_in_search_result VARCHAR,
    area_of_search_condition VARCHAR,
    commute_station_of_search_condition VARCHAR,
    floor VARCHAR,
    rental_fee VARCHAR,
    management_fee VARCHAR,
    security_deposit VARCHAR,
    key_money VARCHAR,
    floor_plan VARCHAR,
    private_area VARCHAR,
    scraping_date  DATETIME
);

-- DROP TABLE temp_room_header;
CREATE TABLE temp_room_header (
    url VARCHAR NOT NULL,
    building_name VARCHAR,
    location VARCHAR,
    walk_to_station VARCHAR,
    age_in_years VARCHAR,
    number_of_floors VARCHAR,
    transfer_in_search_result VARCHAR,
    area_of_search_condition VARCHAR,
    commute_station_of_search_condition VARCHAR,
    floor VARCHAR,
    rental_fee VARCHAR,
    management_fee VARCHAR,
    security_deposit VARCHAR,
    key_money VARCHAR,
    floor_plan VARCHAR,
    private_area VARCHAR,
    scraping_date  DATETIME
);

/* --------- room_raw ------------- */

-- DROP TABLE room_raw;
CREATE TABLE room_raw (
    -- 詳細ページのURL
    url VARCHAR NOT NULL,
    -- suumo物件コード
    suumo_code VARCHAR,
    -- 建物名
    building_name VARCHAR,
    -- 家賃
    rental_fee VARCHAR,
    -- 管理費共益費
    management_fee VARCHAR,
    -- 敷金
    security_deposit VARCHAR,
    -- 礼金
    key_money VARCHAR,
    -- 保証金
    guarantee_deposit VARCHAR,
    -- 敷引償却
    key_money_amortization VARCHAR,
    -- 所在地
    location VARCHAR,
    -- 駅徒歩
    walk_to_station VARCHAR,
    -- 間取り
    floor_plan VARCHAR,
    -- 間取り詳細
    floor_plan_details VARCHAR,
    -- 専有面積
    private_area VARCHAR,
    -- 築年数
    age_in_years VARCHAR,
    -- 築年月
    age_in_months VARCHAR,
    -- 階
    floor VARCHAR,
    -- 階建
    number_of_floors VARCHAR,
    -- 向き
    facing_direction VARCHAR,
    -- 建物種別
    building_type VARCHAR,
    -- 部屋の特徴設備
    features VARCHAR,
    -- 構造
    structure VARCHAR,
    -- 損保
    damage_insurance VARCHAR,
    -- 駐車場
    parking VARCHAR,
    -- 入居（時期）
    move_in VARCHAR,
    -- 取引態様
    transaction_type VARCHAR,
    -- 条件
    conditions VARCHAR,
    -- 取り扱い店舗物件コード
    property_code VARCHAR,
    -- 情報更新日
    info_update_date VARCHAR,
    -- 次回更新日
    next_update_date VARCHAR,
    -- 契約期間
    contract_period VARCHAR,
    -- 備考
    notes VARCHAR,
    -- スクレイピングした日時
    scraping_date DATETIME,
    ---------------------------------
    -- PK制約
    PRIMARY KEY (url)
);

-- DROP TABLE load_room_raw;
CREATE TABLE load_room_raw (
    url VARCHAR NOT NULL,
    suumo_code VARCHAR,
    building_name VARCHAR,
    rental_fee VARCHAR,
    management_fee VARCHAR,
    security_deposit VARCHAR,
    key_money VARCHAR,
    guarantee_deposit VARCHAR,
    key_money_amortization VARCHAR,
    location VARCHAR,
    walk_to_station VARCHAR,
    floor_plan VARCHAR,
    floor_plan_details VARCHAR,
    private_area VARCHAR,
    age_in_years VARCHAR,
    age_in_months VARCHAR,
    floor VARCHAR,
    number_of_floors VARCHAR,
    facing_direction VARCHAR,
    building_type VARCHAR,
    features VARCHAR,
    structure VARCHAR,
    damage_insurance VARCHAR,
    parking VARCHAR,
    move_in VARCHAR,
    transaction_type VARCHAR,
    conditions VARCHAR,
    property_code VARCHAR,
    info_update_date VARCHAR,
    next_update_date VARCHAR,
    contract_period VARCHAR,
    notes VARCHAR,
    scraping_date DATETIME
);

-- DROP TABLE temp_room_raw;
CREATE TABLE temp_room_raw (
    url VARCHAR NOT NULL,
    suumo_code VARCHAR,
    building_name VARCHAR,
    rental_fee VARCHAR,
    management_fee VARCHAR,
    security_deposit VARCHAR,
    key_money VARCHAR,
    guarantee_deposit VARCHAR,
    key_money_amortization VARCHAR,
    location VARCHAR,
    walk_to_station VARCHAR,
    floor_plan VARCHAR,
    floor_plan_details VARCHAR,
    private_area VARCHAR,
    age_in_years VARCHAR,
    age_in_months VARCHAR,
    floor VARCHAR,
    number_of_floors VARCHAR,
    facing_direction VARCHAR,
    building_type VARCHAR,
    features VARCHAR,
    structure VARCHAR,
    damage_insurance VARCHAR,
    parking VARCHAR,
    move_in VARCHAR,
    transaction_type VARCHAR,
    conditions VARCHAR,
    property_code VARCHAR,
    info_update_date VARCHAR,
    next_update_date VARCHAR,
    contract_period VARCHAR,
    notes VARCHAR,
    scraping_date DATETIME
);

/* -- litestream に自動生成されるテーブル -- */

CREATE TABLE _litestream_seq (id INTEGER PRIMARY KEY, seq INTEGER);
CREATE TABLE _litestream_lock (id INTEGER);
