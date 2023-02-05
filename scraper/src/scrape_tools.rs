use std::{env::consts, str::Split};

use crate::utils::config::get_config;

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
    let user_agent = get_config().scraper.user_agent;
    user_agent.replace("$osinfo", consts::OS)
}