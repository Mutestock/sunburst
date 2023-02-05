use std::env;

pub fn db_testing_mode() {
    env::set_var("TEST_MODE_SCRAPER", "1");
}