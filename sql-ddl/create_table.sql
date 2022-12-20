-- room_header definition

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
    scraping_date  DATETIME
);

-- DROP TABLE temp_room_header;
CREATE TABLE temp_room_header (
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
    scraping_date  DATETIME
);

-- litestream に自動生成されるテーブル
CREATE TABLE _litestream_seq (id INTEGER PRIMARY KEY, seq INTEGER);
CREATE TABLE _litestream_lock (id INTEGER);