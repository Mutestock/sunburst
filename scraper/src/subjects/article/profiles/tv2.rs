use std::time::Duration;

use chrono::Local;
use sunburst_models::article::Article;

use crate::{
    handlers::article_handler::article_insert_many,
    scrape_tools::{make_request, ExtractionHelper},
    subjects::article::article_profile::{ArticleProfile, CountryCodes},
    utils::config::CONFIG,
};

pub struct Tv2Scrape;

impl ArticleProfile for Tv2Scrape {
    fn run_sequence(content: String) -> Vec<Article> {
        let desired_content = content.after("Nyhedsoverblik");
        let mut articles: Vec<Article> = Vec::new();
        let article_tags = desired_content
            .split("<article class")
            .skip(1)
            .map(|x| x.to_owned().before("</article>"))
            .collect::<Vec<String>>();

        for article_tag_contents in article_tags {
            let name: String;
            if article_tag_contents.contains("</h3>") {
                name = article_tag_contents.to_owned().before("</h3>").after(">")
            } else if article_tag_contents.contains("</h2>") {
                name = article_tag_contents.to_owned().before("</h2>").after(">")
            } else {
                println!("WARNING. \nPROFILE: tv2\nTHE FOLLOWING ARTICLE TAG WAS NOT RECOGNIZED BY THE SCRAPER: {}", article_tag_contents.to_owned());
                continue;
            }
            let url = &article_tag_contents
                .to_owned()
                .after(" href=\"")
                .before("\"");

            let mut labels = vec![];
            if article_tag_contents.to_owned().contains("</span>") {
                labels.push(article_tag_contents.before("</span>").after(">"));
            }
            articles.push(Article::new(
                &name,
                &Tv2Scrape::get_site(),
                &url,
                &Tv2Scrape::get_language().to_string(),
                &Tv2Scrape::get_scrape_date(),
                None,
                labels,
            ));
        }
        articles
    }

    fn get_site() -> String {
        "TV2".to_owned()
    }

    fn get_language() -> CountryCodes {
        CountryCodes::Denmark
    }

    fn get_scrape_date() -> chrono::NaiveDateTime {
        Local::now().naive_local()
    }
}

impl Tv2Scrape {
    pub async fn run_timed_sequence_loop() {
        tokio::task::spawn(async {
            loop {
                let content = make_request(&CONFIG.scraper.profiles.tv2.url)
                    .await
                    .expect("Could not create request against TV2")
                    .text()
                    .await
                    .expect("Could not extract the text from the TV2 request");
                println!(
                    "{}: Request sent against: {}",
                    Local::now().to_string(),
                    &CONFIG.scraper.profiles.tv2.url
                );
                let extracted_articles = Tv2Scrape::run_sequence(content);
                article_insert_many(&extracted_articles)
                    .await
                    .expect("Could not insert many articles after TV2 scrape");

                tokio::time::sleep(Duration::from_secs(
                    CONFIG.scraper.profiles.tv2.request_frequency_seconds,
                ))
                .await;
            }
        });
    }
}
