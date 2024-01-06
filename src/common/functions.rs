use chrono::{DateTime, Duration, Utc};
use leptos::*;

use super::models::Status;

// Function to calculate and print the time elapsed since the given timestamp
pub fn print_time_since(timestamp: &str) -> String {
    // Parse the input timestamp
    let past_time = match timestamp.parse::<DateTime<Utc>>() {
        Ok(time) => time,
        Err(_e) => return String::from("Unknown"),
    };

    // Get the current time
    let now = Utc::now();

    // Calculate the duration since the given timestamp
    let duration_since = now.signed_duration_since(past_time);

    // Format and return the duration
    format_duration(&duration_since)
}

fn format_duration(duration: &Duration) -> String {
    if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else {
        format!("{} minutes ago", duration.num_minutes())
    }
}

pub fn get_status(timestamp: &str) -> Status {
    match timestamp.parse::<DateTime<Utc>>() {
        Ok(parsed_timestamp) => {
            if Utc::now() < parsed_timestamp {
                Status::Pending
            } else {
                Status::Complete
            }
        }
        Err(_) => Status::Unknown,
    }
}

pub fn convert_to_span(data: String) -> HtmlElement<html::AnyElement> {
    html::span().child(data).into()
}

pub fn convert_to_link(data: String, href: String) -> HtmlElement<html::AnyElement> {
    html::div()
        .attr("class", "w-full text-ellipsis overflow-hidden")
        .child(
            html::a()
                .attr("href", href)
                .attr(
                    "class",
                    "hover:text-granola-orange hover:underline hover:decoration-2",
                )
                .child(data),
        ).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_format_duration_days() {
        let duration = Duration::days(3);
        assert_eq!(format_duration(&duration), "3 days ago");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = Duration::hours(5);
        assert_eq!(format_duration(&duration), "5 hours ago");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::minutes(45);
        assert_eq!(format_duration(&duration), "45 minutes ago");
    }

    #[test]
    fn test_format_duration_mix() {
        let duration = Duration::hours(26);
        assert_eq!(format_duration(&duration), "1 days ago");
    }
}
