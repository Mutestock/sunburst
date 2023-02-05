use chrono::Local;

use crate::subjects::article::article_profile::{ArticleProfile, CountryCodes};


pub struct Tv2Scrape;

impl ArticleProfile for Tv2Scrape {
    fn run_sequence() {
        
    }
    fn extract_name(&self, content: &str) -> String {
        todo!()
    }

    fn extract_site(&self, content: &str) -> String {
        todo!()
    }

    fn extract_url(&self, content: &str) -> String {
        todo!()
    }

    fn get_language(&self) -> CountryCodes {
        CountryCodes::Denmark
    }

    fn get_scrape_date(&self) -> chrono::NaiveDateTime {
        Local::now().naive_local()
    }

    fn extract_submission_date(&self, content: &str) -> Option<chrono::NaiveDateTime> {
        todo!()
    }
}