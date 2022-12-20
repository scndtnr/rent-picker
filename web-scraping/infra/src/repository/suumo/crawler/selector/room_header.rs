/// 賃貸一覧ページのセレクタ

pub fn residence_root() -> String {
    "div[class='cassetteitem']".to_string()
}
pub fn building_name() -> String {
    "div[class='cassetteitem_content-title']".to_string()
}
pub fn location() -> String {
    "li[class='cassetteitem_detail-col1']".to_string()
}
pub fn walk_to_station() -> String {
    "li[class='cassetteitem_detail-col2']".to_string()
}
pub fn age_in_years() -> String {
    "li[class='cassetteitem_detail-col3'] div:nth-child(1)".to_string()
}
pub fn number_of_floors() -> String {
    "li[class='cassetteitem_detail-col3'] div:nth-child(2)".to_string()
}
pub fn transfer_in_search_result() -> String {
    "dd[class='cassetteitem_transfer-body'] li".to_string()
}
pub fn rooms() -> String {
    "table[class='cassetteitem_other'] tbody".to_string()
}
pub fn room_floor() -> String {
    "td:nth-child(3)".to_string()
}
pub fn room_rental_fee() -> String {
    "span[class='cassetteitem_price cassetteitem_price--rent']".to_string()
}
pub fn room_management_fee() -> String {
    "span[class='cassetteitem_price cassetteitem_price--administration']".to_string()
}
pub fn room_security_deposit() -> String {
    "span[class='cassetteitem_price cassetteitem_price--deposit']".to_string()
}
pub fn room_key_money() -> String {
    "span[class='cassetteitem_price cassetteitem_price--gratuity']".to_string()
}
pub fn room_floor_plan() -> String {
    "span[class='cassetteitem_madori']".to_string()
}
pub fn room_private_area() -> String {
    "span[class='cassetteitem_menseki']".to_string()
}
pub fn room_path() -> String {
    "a[href][class='js-cassette_link_href cassetteitem_other-linktext']".to_string()
}
pub fn pagination_parts() -> String {
    "ol[class='pagination-parts'] > li > a".to_string()
}
