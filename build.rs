use std::env;
use std::fs;

fn main() {
    // Read environment variables
    let feature_flag_enabled = match env::var("BERKELEY_FEATURES_ENABLED") {
        Ok(value) => value == "true",  // Check if the value is "true"
        Err(_) => false,  // Default value if environment variable is not set or is invalid
    };

    // Determine the path to the file to be overwritten
    let file_path = "src/config.rs"; // Modify this with the actual path

    // Generate the content to be written to the file
    let content = format!("pub const BERKELEY_FEATURES_ENABLED: bool = {};", feature_flag_enabled);

    // Write the content to the file, overwriting its previous contents
    fs::write(file_path, content).expect("Failed to write file");
}
