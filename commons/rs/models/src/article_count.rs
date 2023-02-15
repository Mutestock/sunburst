use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleCount {
    pub total: i64,
    pub cnt_contained_search_term: i64,
    pub cnt_not_contained_search_term: i64,
}

impl ArticleCount {
    pub fn new(
        total: i64,
        cnt_contained_search_term: i64,
        cnt_not_contained_search_term: i64,
    ) -> Self {
        Self {
            total,
            cnt_contained_search_term,
            cnt_not_contained_search_term,
        }
    }
}
