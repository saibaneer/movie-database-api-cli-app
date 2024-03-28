use reqwest;
use serde_json::Value;
use tokio;
use clap::{Command, Arg};

// Function to fetch movie data
async fn fetch_movie_data(movie_name: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> { // Changed return type to handle different error types
    let url = format!("https://movie-database-alternative.p.rapidapi.com?s={movie_name}&r=json&page=1");

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("X-RapidAPI-Key", api_key)
        .header("X-RapidAPI-Host", "movie-database-alternative.p.rapidapi.com")
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let json: Value = serde_json::from_str(&body)?;
        if let Some(search) = json.get("Search") {
            if let Some(first_result) = search.as_array().and_then(|arr| arr.get(0).cloned()) {
                println!("First result: {:#?}", first_result);
            }
        }
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = Command::new("Movie Fetcher")
        .version("1.0")
        .about("Fetch movie data from a database")
        .arg(Arg::new("movie_name").help("Name of the movie to fetch data for").required(true).index(1))
        .arg(Arg::new("api_key").short('k').long("api-key").help("RapidAPI key").required(true))
        .get_matches();

    let movie_name = matches.get_one::<String>("movie_name").unwrap();
    let api_key = matches.get_one::<String>("api_key").unwrap();

    if let Err(e) = fetch_movie_data(movie_name, api_key).await {
        eprintln!("Error fetching movie data: {}", e);
    }
}
