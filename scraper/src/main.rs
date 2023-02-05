use scraper::handlers::article_handler::article_insert_many;
use scraper::scrape_tools::make_request;
use scraper::subjects::article::article_profile::ArticleProfile;
use scraper::subjects::article::profiles::tv2::Tv2Scrape;
use scraper::utils::config::get_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = make_request(&get_config().scraper.profiles.tv2.url)
        .await?
        .text()
        .await?;
    let extracted_articles = Tv2Scrape::run_sequence(content);
    println!("{:?}", extracted_articles);
    article_insert_many(&extracted_articles).await?;
    Ok(())
}
