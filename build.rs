use std::{env, fs};

fn main() {
    let feature_flag_enabled = match env::var("BERKELEY_FEATURES_ENABLED") {
        Ok(value) => value == "true",
        Err(_) => false,
    };

    let file_path = "src/config.rs";

    let content = format!(
        "pub const BERKELEY_FEATURES_ENABLED: bool = {};",
        feature_flag_enabled
    );

    fs::write(file_path, content).expect("Failed to write file");
}
