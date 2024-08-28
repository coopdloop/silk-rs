use log::{debug, error, info};
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
    // Initialize logger
    env_logger::init();

    let client = Client::new();
    let mut pages_to_visit = vec![site_url.to_string()];
    let mut selectors_with_tags: Vec<(String, Selector)> = Vec::new();

    for tag in tags_to_find {
        match Selector::parse(&tag) {
            Ok(selector) => selectors_with_tags.push((tag.clone(), selector)),
            Err(e) => {
                error!("Failed to parse selector '{}': {}", tag, e);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid selector: {}", tag),
                )));
            }
        }
    }

    for i in 0..spider_count {
        if pages_to_visit.is_empty() {
            info!("No more pages to visit. Ending scraping.");
            break;
        }

        let current_url = pages_to_visit.pop().unwrap();
        info!("Scraping page {}: {}", i + 1, current_url);

        let response = match client.get(&current_url).send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Failed to fetch {}: {}", current_url, e);
                continue;
            }
        };

        let html_content = match response.text().await {
            Ok(content) => content,
            Err(e) => {
                error!("Failed to get content from {}: {}", current_url, e);
                continue;
            }
        };

        let document = Html::parse_document(&html_content);

        for (tag, selector) in &selectors_with_tags {
            let elements = document.select(selector);
            for element in elements {
                let text = element
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                info!("Found element for selector '{}': {}", tag, text);

                if let Some(href) = element.value().attr("href") {
                    debug!("Found href: {}", href);
                    if spider_paths && pages_to_visit.len() < spider_count as usize {
                        pages_to_visit.push(href.to_string());
                    }
                }
            }
        }

        // Randomize request timing if needed
        if rand_reqs {
            let random_delay = rand::thread_rng().gen_range(500..2000);
            debug!("Random delay: {} ms", random_delay);
            tokio::time::sleep(Duration::from_millis(random_delay)).await;
        }
    }
    Ok(())
}
