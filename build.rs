use std::env;

fn main() {
    let graphql_url =
        env::var("GRAPHQL_URL").unwrap_or("https://api.minasearch.com/graphql".into());
    let rest_url = env::var("REST_URL").unwrap_or("https://api.minasearch.com".into());
    let commit_hash = env::var("VERSION").unwrap_or_else(|_| "version".to_string());

    println!("cargo:rustc-env=GRAPHQL_URL={}", graphql_url);
    println!("cargo:rustc-env=REST_URL={}", rest_url);
    println!("cargo:rustc-env=COMMIT_HASH={}", commit_hash);
}
