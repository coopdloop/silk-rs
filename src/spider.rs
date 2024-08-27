use rand::Rng;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;
use std::vec;

// Function handling web scraping with params
pub async fn scrape_website(
    site_url: &str,
    spider_paths: bool,
    spider_count: u8,
    tags_to_find: Vec<String>,
    rand_reqs: bool,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut pages_to_visit = vec![site_url.to_string()];

    // This vector will hold both the original strings and the selectors
    let mut selectors_with_tags: Vec<(String, Selector)> = Vec::new();

    // Pre-parse the selectors and store them with their corresponding Strings
    for tag in tags_to_find {
        let tag_clone = tag.clone();
        let selector = Selector::parse(&tag_clone)?; // Parse the selector
        selectors_with_tags.push((tag_clone, selector)); // Clone the tag and store both
    }

    //
    for i in 0..spider_count {
        if pages_to_visit.is_empty() {
            break;
        }

        let current_url = pages_to_visit.pop().unwrap();
        println!("Scraping page {}: {}", i + 1, current_url);

        let response = client.get(&current_url).send().await?;
        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);

        for (_, selector) in &selectors_with_tags {
            let elements = document.select(selector);

            for element in elements {
                let text = element.text().collect::<Vec<_>>().join(" ");
                println!("Found element: {}", text);

                // Get attrs like href or src
                if let Some(href) = element.value().attr("href") {
                    println!("Found href: {}", href);
                    if spider_paths && pages_to_visit.len() < spider_count as usize {
                        pages_to_visit.push(href.to_string());
                    }
                }
            }
        }

        // Randomize request timing if needed
        if rand_reqs {
            let random_delay = rand::thread_rng().gen_range(500..2000);
            tokio::time::sleep(Duration::from_millis(random_delay)).await;
        }
    }
    Ok(())
}
