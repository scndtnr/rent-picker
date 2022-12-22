/// 賃貸詳細ページのセレクタ

// タイトル

pub fn building_name() -> String {
    "h1[class='section_h1-header-title']".to_string()
}

// 料金概要

pub fn rental_fee() -> String {
    "span[class='property_view_note-emphasis']".to_string()
}
pub fn management_fee() -> String {
    "div[class='property_view_note-info'] > div:nth-child(1)  > span:nth-child(2)".to_string()
}
pub fn security_deposit() -> String {
    "div[class='property_view_note-info'] > div:nth-child(2)  > span:nth-child(1)".to_string()
}
pub fn key_money() -> String {
    "div[class='property_view_note-info'] > div:nth-child(2)  > span:nth-child(2)".to_string()
}
pub fn guarantee_deposit() -> String {
    "div[class='property_view_note-info'] > div:nth-child(2)  > span:nth-child(3)".to_string()
}
pub fn key_money_amortization() -> String {
    "div[class='property_view_note-info'] > div:nth-child(2)  > span:nth-child(4)".to_string()
}

// 建物概要

pub fn about_building_table() -> String {
    "table[class='property_view_table']".to_string()
}

// 部屋の特徴・設備

pub fn features() -> String {
    "div[id='contents'] div:nth-child(1) > ul[class='inline_list']".to_string()
}

// 物件概要

pub fn about_room_table() -> String {
    "table[class='data_table table_gaiyou']".to_string()
}
