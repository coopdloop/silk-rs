use reqwest::Client;
// Import the `Client` type from the `reqwest` crate, which allows making HTTP requests.

use scraper::{Html, Selector};
// Import the `Html` and `Selector` types from the `scraper` crate for parsing and selecting elements from the HTML.

use std::error::Error;
// Import the `Error` trait from the standard library for error handling.

#[tokio::main]
// The `#[tokio::main]` attribute allows this function to be the entry point for an asynchronous application using the Tokio runtime.
async fn main() -> Result<(), Box<dyn Error>> {
    // The `main` function is asynchronous and returns a `Result` type. `Box<dyn Error>` allows returning any error type.

    let client = Client::new();
    // Create a new instance of the `Client` from `reqwest` for making HTTP requests.

    let response = client
        .get("https://scrapeme.live/shop/")
        // Send a GET request to the specified URL.
        .send()
        .await?;
    // Await the response asynchronously and propagate errors using the `?` operator.

    let html_content = response.text().await?;
    // Extract the HTML content from the response as text and propagate errors using the `?` operator.

    let document = Html::parse_document(&html_content);
    // Parse the HTML content into a `Html` document that can be queried.

    let product_selector = Selector::parse("li.product")?;
    // Create a selector to match `<li>` elements with the class `product`. The `?` operator handles any parsing errors.

    let products = document.select(&product_selector);
    // Use the selector to find all matching elements (i.e., products) in the HTML document.

    for product in products {
        // Iterate over each product found in the document.

        let url = product
            .select(&Selector::parse("a")?)
            // Select the first `<a>` element within the product.
            .next()
            // Get the first element from the selection, if it exists.
            .and_then(|a| a.value().attr("href"))
            // If the element exists, get the value of the `href` attribute.
            .map_or_else(|| "N/A".to_string(), str::to_owned);
        // If `href` exists, convert it to a `String`, otherwise use "N/A" as a default value.

        let image_url = product
            .select(&Selector::parse("img")?)
            // Select the first `<img>` element within the product.
            .next()
            // Get the first element from the selection, if it exists.
            .and_then(|img| img.value().attr("src"))
            // If the element exists, get the value of the `src` attribute.
            .map_or_else(|| "N/A".to_string(), str::to_owned);
        // If `src` exists, convert it to a `String`, otherwise use "N/A" as a default value.

        let product_name = product
            .select(&Selector::parse("h2")?)
            // Select the first `<h2>` element within the product.
            .next()
            // Get the first element from the selection, if it exists.
            .map_or_else(|| "Unknown".to_string(), |h2| h2.text().collect::<String>());
        // If the `<h2>` exists, collect its text content into a `String`, otherwise use "Unknown" as a default value.

        let product_price = product
            .select(&Selector::parse(".price")?)
            // Select the first element with the class `price` within the product.
            .next()
            // Get the first element from the selection, if it exists.
            .map_or_else(
                || "Unavailable".to_string(),
                |price| price.text().collect::<String>(),
            );
        // If the price element exists, collect its text content into a `String`, otherwise use "Unavailable" as a default value.

        println!(
            "\nname = {:?}\nprice = {:?}\nurl = {:?}\nimage_url = {:?}",
            product_name,
            product_price,
            url,
            image_url // Print the product's name, price, URL, and image URL in a formatted and readable way.
        );
    }

    Ok(())
    // Return `Ok(())` to indicate that the program finished successfully.
}
