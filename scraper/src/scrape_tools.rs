use std::env::consts;

use crate::utils::config::CONFIG;



pub trait ExtractionHelper {
    fn before(self, search_term: &str) -> String;
    fn after(self, search_term: &str) -> String;
    fn at(self, search_term: &str, index: usize) -> String;
}

impl ExtractionHelper for String {
    fn before(self, search_term: &str) -> String {
        self.split(search_term).next().unwrap().to_owned()
    }

    fn after(self, search_term: &str) -> String {
        self.split(search_term).last().unwrap().to_owned()
    }

    fn at(self, search_term: &str, index: usize) -> String {
        self.split(search_term).nth(index).unwrap().to_owned()
    }
}

pub fn construct_user_agent() -> String {
    let user_agent = &CONFIG.scraper.user_agent;
    let info = os_info::get();
    user_agent.replace("$osinfo", &format!("{}/{}/{}", consts::OS, info.os_type(), info.version()))
}

pub async fn make_request(url_to_scrape: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .user_agent(construct_user_agent())
        .build()?;
    client.get(url_to_scrape).send().await
}
