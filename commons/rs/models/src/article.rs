use chrono::naive::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub name: String,
    pub site: String,
    pub url: String,
    pub country_code: String,
    pub scrape_date: NaiveDateTime,
    pub submission_date: Option<NaiveDateTime>,
    pub tags_and_categories: Vec<String>,
}

impl Article {
    pub fn new(
        name: &str,
        site: &str,
        url: &str,
        country_code: &str,
        scrape_date: &NaiveDateTime,
        submission_date: Option<NaiveDateTime>,
        tags_and_categories: Vec<String>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            site: site.to_owned(),
            url: url.to_owned(),
            country_code: country_code.to_owned(),
            scrape_date: scrape_date.clone(),
            submission_date,
            tags_and_categories
        }
    }
}
