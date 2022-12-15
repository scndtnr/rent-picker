-- room_header definition

-- DROP TABLE room_header;
CREATE TABLE room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR NOT NULL, 
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    created_at DATETIME, 
    PRIMARY KEY (url)
);

-- DROP TABLE load_room_header;
CREATE TABLE load_room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR NOT NULL, 
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    created_at DATETIME
);

-- DROP TABLE temp_room_header;
CREATE TABLE temp_room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR NOT NULL, 
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    created_at DATETIME
);