-- room_header definition

CREATE TABLE room_header (
    url VARCHAR NOT NULL, 
    residence_title VARCHAR, 
    residence_transfer VARCHAR, 
    residence_area VARCHAR, 
    residence_station VARCHAR, 
    created_at DATETIME, 
    PRIMARY KEY (url)
);
