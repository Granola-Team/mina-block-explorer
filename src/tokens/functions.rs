use super::models::{TokenData, TokenDataSortBy};
use crate::common::models::MyError;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub async fn load_data(
    limit: u64,
    name: Option<String>,
    id: Option<String>,
    owner: Option<String>,
    sort_by: Option<TokenDataSortBy>,
    ascending: bool,
) -> Result<(Vec<TokenData>, i64), MyError> {
    let client = Client::new();

    // Build the base URL with select
    let mut url =
        String::from("https://owdfifqnnanbqwbuyzsj.supabase.co/rest/v1/zkapp_tokens?select=*");

    // Add limit if provided
    url.push_str(&format!("&limit={:?}", limit));

    // Add search filters if provided
    if let Some(name) = name {
        url.push_str(&format!(
            "&name=ilike.{}",
            urlencoding::encode(&format!("%{}%", name))
        ));
    }
    if let Some(id) = id {
        url.push_str(&format!("&id=eq.{}", id));
    }
    if let Some(owner) = owner {
        url.push_str(&format!("&owner=eq.{}", owner));
    }

    // Add sorting if provided
    if let Some(sort) = sort_by {
        url.push_str(&format!(
            "&order={}.{}",
            sort.as_str(),
            if ascending { "asc" } else { "desc" }
        ));
    }

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(
        "apikey",
        // this is fine to be public
        HeaderValue::from_str(
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6Im93ZGZpZnFubmFuYnF3YnV5enNqIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Mzc0NzMxMTEsImV4cCI6MjA1MzA0OTExMX0.i2FHhUUEZDbmAXSH3uz8Yt7D09PJvdFILlowrwbz5ro"
        ).map_err(|e| MyError::UrlParseError(e.to_string()))?
    );

    // Add Prefer header for exact count
    headers.insert("Prefer", HeaderValue::from_static("count=exact"));

    // Make the request
    let response = client.get(&url).headers(headers).send().await?;

    // Get total count from content-range header
    let total_count = response
        .headers()
        .get("content-range")
        .and_then(|h| h.to_str().ok())
        .and_then(|range| {
            range
                .split('/')
                .nth(1)
                .and_then(|count| count.parse::<i64>().ok())
        })
        .unwrap_or(0);
    println!("total_count: {total_count}");

    // Parse the JSON response
    let data = response
        .json::<Vec<TokenData>>()
        .await
        .map_err(|e| MyError::ParseError(e.to_string()))?;

    Ok((data, total_count))
}
