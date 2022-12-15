-- room_header definition

-- DROP TABLE room_header;
CREATE TABLE room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR, 
    residence_address VARCHAR,
    residence_nearest_station VARCHAR,
    residence_age VARCHAR,
    residence_floors VARCHAR,
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    room_floor VARCHAR,
    room_rent_price VARCHAR,
    room_condo_fee VARCHAR,
    room_deposit VARCHAR,
    room_key_money VARCHAR,
    room_layout VARCHAR,
    room_exclusive_area VARCHAR,
    created_at DATETIME, 
    PRIMARY KEY (url)
);

-- DROP TABLE load_room_header;
CREATE TABLE load_room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR, 
    residence_address VARCHAR,
    residence_nearest_station VARCHAR,
    residence_age VARCHAR,
    residence_floors VARCHAR,
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    room_floor VARCHAR,
    room_rent_price VARCHAR,
    room_condo_fee VARCHAR,
    room_deposit VARCHAR,
    room_key_money VARCHAR,
    room_layout VARCHAR,
    room_exclusive_area VARCHAR,
    created_at DATETIME
);

-- DROP TABLE temp_room_header;
CREATE TABLE temp_room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR, 
    residence_address VARCHAR,
    residence_nearest_station VARCHAR,
    residence_age VARCHAR,
    residence_floors VARCHAR,
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    room_floor VARCHAR,
    room_rent_price VARCHAR,
    room_condo_fee VARCHAR,
    room_deposit VARCHAR,
    room_key_money VARCHAR,
    room_layout VARCHAR,
    room_exclusive_area VARCHAR,
    created_at DATETIME
);