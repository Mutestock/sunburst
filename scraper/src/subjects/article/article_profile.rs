use chrono::NaiveDateTime;

pub trait ArticleProfile{
    fn run_sequence();
    fn extract_name(&self, content: &str) -> String;
    fn extract_site(&self, content: &str) -> String;
    fn extract_url(&self, content: &str) -> String;
    fn get_language(&self) -> CountryCodes;
    fn get_scrape_date(&self) -> NaiveDateTime;
    fn extract_submission_date(&self, content: &str) -> Option<NaiveDateTime>;
}

pub enum CountryCodes{
    Denmark,
}


impl ToString for CountryCodes{
    fn to_string(&self) -> String {
        match self {
            CountryCodes::Denmark => "dkk",
        }.to_owned()
    }
}