use chrono::Local;
use rs_models::article::Article;

use crate::{
    scrape_tools::ExtractionHelper,
    subjects::article::article_profile::{ArticleProfile, CountryCodes},
};

pub struct Tv2Scrape;

impl ArticleProfile for Tv2Scrape {
    fn run_sequence(content: String) {
        let desired_content = content.after("Nyhedsoverblik");
        let mut articles: Vec<Article> = Vec::new();
        articles.push(Tv2Scrape::get_overview_article(&desired_content));
        articles.extend(Tv2Scrape::get_lesser_grid_articles_below_overview(
            &desired_content,
        ));
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
    fn get_overview_article(content: &str) -> Article {
        let isolated_content = content.to_owned().before("tc_teaser__tagline__text");
        let name = &isolated_content.to_owned().before("</h3>").after(">");
        let url = &isolated_content.to_owned().after("href=\"").before("\"");

        Article::new(
            name,
            &Tv2Scrape::get_site(),
            url,
            &Tv2Scrape::get_language().to_string(),
            &Tv2Scrape::get_scrape_date(),
            None,
            vec![],
        )
    }

    fn get_lesser_grid_articles_below_overview(content: &str) -> Vec<Article> {
        /*
           ------------------------------------------------
           |   ((*))         Overview                     |
           ------------------------------------------------
           ---------------------      ---------------------
        -> |                   |   -> |                   |
           ---------------------      ---------------------
           */

        let isolated_content = content
            .to_owned()
            .at("<div class=\"tc_grid\"", 1)
            .before("</div>");

        let mut articles: Vec<Article> = Vec::new();

        for article_content in isolated_content.split("article").skip(1) {
            let name = article_content.to_owned().before("</h2>").after(">");
            let url = article_content.to_owned().after("href=\"").before("\"");
            let tag = article_content.to_owned().before("</span>").after(">");

            articles.push(Article::new(
                &name,
                &Tv2Scrape::get_site(),
                &url,
                &Tv2Scrape::get_language().to_string(),
                &Tv2Scrape::get_scrape_date(),
                None,
                vec![tag],
            ));
        }
        articles
    }
}
