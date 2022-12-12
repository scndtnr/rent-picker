pub trait SuumoSelector {
    // トップページのセレクタ
    fn kanto_title(&self) -> &str {
        "h1[class='hantitle-txt']"
    }

    // 賃貸一覧ページのセレクタ

    fn residence_root(&self) -> &str {
        "div[class='cassetteitem']"
    }
    fn residence_name(&self) -> &str {
        "div[class='cassetteitem_content-title']"
    }
    fn residence_transfer(&self) -> &str {
        "dd[class='cassetteitem_transfer-body'] li"
    }
    fn room_path(&self) -> &str {
        "a[href][target='_blank']"
    }
    fn pagination_parts(&self) -> &str {
        "ol[class='pagination-parts'] > li > a"
    }

    // 賃貸詳細ページのセレクタ
}
