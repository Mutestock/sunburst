use chrono::NaiveDateTime;
use rs_models::article::Article;

pub trait ArticleProfile{
    fn run_sequence(content: String) -> Vec<Article>;
    fn get_site() -> String;
    fn get_language() -> CountryCodes;
    fn get_scrape_date() -> NaiveDateTime;
}

pub enum CountryCodes{
    Denmark,
}


impl ToString for CountryCodes{
    fn to_string(&self) -> String {
        match self {
            CountryCodes::Denmark => "dk",
        }.to_owned()
    }
}