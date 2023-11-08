use web_scrape_lib::web_scraper;
use web_scraper::scrape;
use tokio;

#[tokio::main]
async fn main() {
    //keyword to be extracted
    let content = "Doctor".to_string();

    let scraped_content = scrape(&content).await;

    //handling the result of the scraping process
    if let Err(e) = scraped_content {
        println!("{}",e.to_string());
    }
    else if let Ok(_) = scraped_content {
        println!("Data successfully scraped to Scraped.txt");
    }
}
