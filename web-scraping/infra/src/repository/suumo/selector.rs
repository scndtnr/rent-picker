#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoSelector {
    pub top: SuumoTopPageSelector,
    pub list: SuumoListPageSelector,
    pub room: SuumoRoomPageSelector,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoTopPageSelector {
    pub kanto_title: String,
}

impl Default for SuumoTopPageSelector {
    fn default() -> Self {
        Self {
            kanto_title: "h1[class='hantitle-txt']".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoListPageSelector {
    pub kanto_title: String,
}

impl Default for SuumoListPageSelector {
    fn default() -> Self {
        Self {
            kanto_title: "h1[class='hantitle-txt']".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuumoRoomPageSelector {
    pub kanto_title: String,
}

impl Default for SuumoRoomPageSelector {
    fn default() -> Self {
        Self {
            kanto_title: "h1[class='hantitle-txt']".to_string(),
        }
    }
}
