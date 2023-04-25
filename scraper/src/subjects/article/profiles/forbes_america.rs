use chrono::{Local, NaiveDateTime};
use sunburst_models::article::Article;

use crate::{
    handlers::article_handler::article_insert_many,
    scrape_tools::{make_request, write_response, ExtractionHelper},
    subjects::article::article_profile::{ArticleProfile, CountryCodes},
    utils::config::CONFIG,
};

pub struct ForbesAmericaScrape;

impl ArticleProfile for ForbesAmericaScrape {
    fn run_sequence(content: String) -> Vec<sunburst_models::article::Article> {
        let mut articles = vec![];
        let desired_content = &content
            .after("<body>")
            .after("<main class")
            .before("</main>");

        write_response(&desired_content, "forbes_america_desired_content.html");

        let happening_now_carousel_content = desired_content
            .to_owned()
            .after("<div class=\"happening-now\">")
            .after("<fbs-carousel")
            .before("</fbs-carousel>");

        let popular_tab_content = desired_content
            .to_owned()
            .after("<ul class=\"data-viz__list data-viz__list--international\">")
            .before("</ul>");

        let editors_picks_content = desired_content
            .to_owned()
            .after("<div class=\"editors-picks\">")
            .before("class=\"editors-picks__see-all");

        articles.extend(
            ForbesAmericaScrape::extract_happening_now_carousel_articles(
                happening_now_carousel_content,
            ),
        );

        articles.extend(ForbesAmericaScrape::extract_popular_tab_articles(
            popular_tab_content,
        ));

        articles.extend(ForbesAmericaScrape::extract_editors_picks(
            editors_picks_content,
        ));

        articles
    }

    fn get_site() -> String {
        "Forbes America".to_owned()
    }

    fn get_country_code() -> crate::subjects::article::article_profile::CountryCodes {
        CountryCodes::UnitedStates
    }

    fn get_scrape_date() -> chrono::NaiveDateTime {
        Local::now().naive_local()
    }
}

impl ForbesAmericaScrape {
    pub async fn run_timed_sequence_loop() {
        let content = make_request(&CONFIG.scraper.profiles.forbes_america.url)
            .await
            .expect("Could not create request against TV2")
            .text()
            .await
            .expect("Could not extract the text from the TV2 request");
        println!(
            "{}: Request sent against: {}",
            Local::now().to_string(),
            &CONFIG.scraper.profiles.forbes_america.url
        );
        write_response(&content, "forbes_america_test_out.html");
        let extracted_articles = ForbesAmericaScrape::run_sequence(content);
        article_insert_many(&extracted_articles)
            .await
            .expect("Could not insert many articles after TV2 scrape");
    }

    fn extract_popular_tab_articles(content: String) -> Vec<Article> {
        let mut articles = vec![];
        let entries = content.split("<li data-index=\"").skip(1);
        for entry in entries {
            let a_tag = &entry.to_owned().after("<a").before("</a>");
            let url = a_tag.to_owned().after("href=\"").before("\"");
            let name = a_tag.to_owned().after(">");
            let submission_date = ForbesAmericaScrape::extract_submission_date(&url);

            articles.push(Article {
                name: name,
                site: ForbesAmericaScrape::get_site(),
                url: url,
                country_code: ForbesAmericaScrape::get_country_code().to_string(),
                scrape_date: ForbesAmericaScrape::get_scrape_date(),
                submission_date,
                tags_and_categories: vec![],
            });
        }

        articles
    }

    fn extract_editors_picks(content: String) -> Vec<Article> {
        let mut articles = Vec::new();

        let picks = content.split("class=\"editors-pick").skip(1);
        // The last pane is a "see all" section
        for pick in picks.into_iter() {
            let a_tag = &pick.to_owned().after("<a class=\"").before("</a>");
            let url = a_tag.to_owned().after("href=\"").before("\"");
            let name = a_tag.to_owned().after(">");
            let submission_date = ForbesAmericaScrape::extract_submission_date(&url);

            articles.push(Article {
                name,
                site: ForbesAmericaScrape::get_site(),
                url,
                country_code: ForbesAmericaScrape::get_country_code().to_string(),
                scrape_date: ForbesAmericaScrape::get_scrape_date(),
                submission_date: submission_date,
                tags_and_categories: vec![],
            })
        }

        articles
    }

    fn extract_happening_now_carousel_articles(content: String) -> Vec<Article> {
        let mut articles = vec![];
        let content = content.after("<div class=\"fbs-slider__slides\"");
        let slides = content.split("<div class=\"fbs-slider__slide").skip(1);

        for slide in slides.into_iter() {
            let name = slide
                .to_owned()
                .after("<a class=\"happening__title")
                .before("</a>")
                .after(">");
            let url = slide.to_owned().after("href=\"").before("\"");
            let submission_date: Option<NaiveDateTime> =
                ForbesAmericaScrape::extract_submission_date(&url);

            articles.push(Article {
                name,
                site: ForbesAmericaScrape::get_site(),
                url,
                country_code: ForbesAmericaScrape::get_country_code().to_string(),
                scrape_date: ForbesAmericaScrape::get_scrape_date(),
                submission_date,
                tags_and_categories: vec![],
            });
        }

        articles
    }

    fn extract_submission_date(url: &str) -> Option<NaiveDateTime> {
        let intermediate_url_extraction_content = url.to_owned().after("sites/");
        let slashes_content = &intermediate_url_extraction_content
            .split("/")
            .collect::<Vec<&str>>()[1..4];
        if slashes_content.len() != 3 {
            return None;
        }
        let all_entries_are_numbers = slashes_content.iter().all(|x| match x.parse::<usize>() {
            Ok(_) => true,
            Err(_) => false,
        });
        if !all_entries_are_numbers {
            return None;
        }
        let year = slashes_content.get(0).unwrap();
        let month = slashes_content.get(1).unwrap();
        let day = slashes_content.get(2).unwrap();

        let fmt = "%Y-%m-%dT%H:%M:%S%.f";
        let parsed = NaiveDateTime::parse_from_str(
            &format!("{}-{}-{}T12:00:00.000000000", year, month, day),
            fmt,
        );

        match parsed {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
