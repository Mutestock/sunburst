use std::time::Duration;

use chrono::Local;
use sunburst_models::article::Article;

use crate::{
    handlers::article_handler::article_insert_many,
    scrape_tools::{make_request, ExtractionHelper},
    subjects::article::article_profile::{ArticleProfile, CountryCodes},
    utils::config::CONFIG,
};

pub struct DrScrape;

impl ArticleProfile for DrScrape {
    fn run_sequence(content: String) -> Vec<Article> {
        let desired_content = content.after("<body").before("<footer");
        let mut articles: Vec<Article> = Vec::new();
        let teasers = desired_content.split("\"dre-teaser\"").skip(1);
        for teaser in teasers {
            let name = teaser.to_owned().after("aria-label=\"").before("\"");
            let mut url = teaser.to_owned().after("href=\"").before("\"");
            let label = teaser
                .to_owned()
                .after("dre-teaser-meta-label")
                .after(">")
                .before("<");
            let labels = vec![label];
            articles.push(Article::new(
                &name,
                &DrScrape::get_site(),
                &{
                    let url_prefix = "https://www.dr.dk";
                    if !url.contains(url_prefix) {
                        url = format!("{}{}", url_prefix, &url);
                    }
                    url
                },
                &DrScrape::get_country_code().to_string(),
                &DrScrape::get_scrape_date(),
                None,
                labels,
            ));
        }
        articles
    }

    fn get_site() -> String {
        "DR".to_owned()
    }

    fn get_country_code() -> crate::subjects::article::article_profile::CountryCodes {
        CountryCodes::Denmark
    }

    fn get_scrape_date() -> chrono::NaiveDateTime {
        Local::now().naive_local()
    }
}

impl DrScrape {
    pub async fn run_timed_sequence_loop() {
        tokio::task::spawn(async {
            loop {
                let content = make_request(&CONFIG.scraper.profiles.dr.url)
                    .await
                    .expect("Could not create request against DR")
                    .text()
                    .await
                    .expect("Could not extract the text from the DR request");
                println!(
                    "{}: Request sent against: {}",
                    Local::now().to_string(),
                    &CONFIG.scraper.profiles.dr.url
                );
                let extracted_articles = DrScrape::run_sequence(content);
                article_insert_many(&extracted_articles)
                    .await
                    .expect("Could not insert many articles after DR scrape");

                tokio::time::sleep(Duration::from_secs(
                    CONFIG.scraper.profiles.dr.request_frequency_seconds,
                ))
                .await;
            }
        });
    }
}
