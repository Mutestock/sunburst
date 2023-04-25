use chrono::NaiveDateTime;
use sunburst_models::article::Article;

use super::profiles::{dr::DrScrape, forbes_america::ForbesAmericaScrape, tv2::Tv2Scrape};

pub trait ArticleProfile {
    fn run_sequence(content: String) -> Vec<Article>;
    fn get_site() -> String;
    fn get_country_code() -> CountryCodes;
    fn get_scrape_date() -> NaiveDateTime;
}

pub enum CountryCodes {
    Denmark,
    UnitedStates,
}

impl ToString for CountryCodes {
    fn to_string(&self) -> String {
        match self {
            CountryCodes::Denmark => "dk",
            CountryCodes::UnitedStates => "us",
        }
        .to_owned()
    }
}

pub async fn initiate_article_profile_scrapers() {
    Tv2Scrape::run_timed_sequence_loop().await;
    DrScrape::run_timed_sequence_loop().await;
    ForbesAmericaScrape::run_timed_sequence_loop().await;
}
