use crate::spider;
use std::error::Error;

// Function that sets up the parameters and calls the scrape_website function
pub async fn perform_web_scraping() -> Result<(), Box<dyn Error>> {
    // Define parameters for the web scraping
    let site_url = "https://scrapeme.live/shop/";
    let spider_paths = true;
    let spider_count = 5;
    // Define tags as Vec<String>
    let tags_to_find = vec![
        "li.product".to_string(),
        "h2".to_string(),
        "img".to_string(),
        ".price".to_string(),
        "a".to_string(),
    ];

    let randomize_requests = true;

    // Call the scrape_website function from the spider module
    spider::scrape_website(
        site_url,
        spider_paths,
        spider_count,
        tags_to_find,
        randomize_requests,
    )
    .await?;

    Ok(())
}
