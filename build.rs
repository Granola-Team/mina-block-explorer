use std::{env, fs};

fn main() {
    let graphql_url = env::var("GRAPHQL_URL").unwrap_or_else(|_| "https://graphql.minaexplorer.com".to_string());
    let rest_url = env::var("REST_URL").unwrap_or_else(|_| "https://api.minaexplorer.com".to_string());
    println!("cargo:rustc-env=GRAPHQL_URL={}", graphql_url);
    println!("cargo:rustc-env=REST_URL={}", rest_url);

    let feature_flag_enabled = match env::var("BERKELEY_FEATURES_ENABLED") {
        Ok(value) => value == "true",
        Err(_) => false,
    };

    let file_path = "src/config.rs";

    let content = format!(
        "pub const BERKELEY_FEATURES_ENABLED: bool = {};\n",
        feature_flag_enabled
    );

    fs::write(file_path, content).expect("Failed to write file");
}
