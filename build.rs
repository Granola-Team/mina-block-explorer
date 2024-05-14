use std::env;

fn main() {
    let graphql_url =
        env::var("GRAPHQL_URL").unwrap_or_else(|_| "https://graphql.minaexplorer.com".to_string());
    let rest_url =
        env::var("REST_URL").unwrap_or_else(|_| "https://api.minaexplorer.com".to_string());
    let berkeley_feature_flag =
        env::var("BERKELEY_FEATURES_ENABLED").unwrap_or_else(|_| "false".to_string());

    println!("cargo:rustc-env=GRAPHQL_URL={}", graphql_url);
    println!("cargo:rustc-env=REST_URL={}", rest_url);
    println!(
        "cargo:rustc-env=BERKELEY_FEATURES_ENABLED={}",
        berkeley_feature_flag
    );
}
