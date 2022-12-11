#[derive(Clone, Debug)]
pub struct SuumoSelector {
    pub kanto_title: String,
}

impl Default for SuumoSelector {
    fn default() -> Self {
        Self {
            kanto_title: "h1[class='hantitle-txt']".to_string(),
        }
    }
}
