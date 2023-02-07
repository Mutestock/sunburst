use chrono::Local;
use sunburst_models::article::Article;

use crate::{
    scrape_tools::ExtractionHelper,
    subjects::article::article_profile::{ArticleProfile, CountryCodes},
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
