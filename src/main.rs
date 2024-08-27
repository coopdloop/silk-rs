mod spider;
mod web;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Call the perform_web_scraping function from the web module
    web::perform_web_scraping().await?;

    Ok(())
}
