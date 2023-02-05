use chrono::naive::NaiveDateTime;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub name: String,
    pub site: String,
    pub url: String,
    pub language: String,
    pub scrape_data: NaiveDateTime,
    pub submission_date: Option<NaiveDateTime>,
}

impl Article {
    pub fn new(
        name: &str,
        site: &str,
        url: &str,
        language: &str,
        scrape_data: &NaiveDateTime,
        submission_date: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            site: site.to_owned(),
            url: url.to_owned(),
            language: language.to_owned(),
            scrape_data: scrape_data.clone(),
            submission_date,
        }
    }
}