use scraper::subjects::article::profiles::{tv2::Tv2Scrape, dr::DrScrape};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Tv2Scrape::run_timed_sequence_loop().await;
    DrScrape::run_timed_sequence_loop().await;

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }
    
    Ok(())
}
