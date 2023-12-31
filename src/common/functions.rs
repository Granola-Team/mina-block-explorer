use chrono::{DateTime, Duration, Utc};
use leptos::*;

use super::models::*;

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

pub fn convert_to_pill(data: String, pill_variant: PillVariant) -> HtmlElement<html::AnyElement> {
    let value_class_str_base = "text-white p-0.5 text-sm";
    let pill_class_str_base = format!("{} {}", value_class_str_base, "px-2 rounded-full");

    let pill_class_str = match pill_variant {
        PillVariant::Green => format!("{} {}", pill_class_str_base.to_owned(), "bg-pill-green"),
        PillVariant::Blue => format!("{} {}", pill_class_str_base.to_owned(), "bg-pill-blue"),
        PillVariant::Orange => {
            format!("{} {}", pill_class_str_base.to_owned(), "bg-granola-orange")
        }
    };
    html::span()
        .attr("class", pill_class_str)
        .child(data)
        .into()
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
        )
        .into()
}

pub fn x_surrounding_pages(x: usize, l: usize) -> Vec<Vec<usize>> {
    let mut preceding = Vec::new();
    let mut following = Vec::new();

    // Start with calculating preceding pages
    let mut start = if x > 3 { x - 3 } else { 1 };
    while start < x {
        preceding.push(start);
        start += 1;
    }

    // Calculate following pages, aiming for a total of 6 pages
    let total_required = 6 - preceding.len();
    let mut end = x + 1;
    while end <= l && following.len() < total_required {
        following.push(end);
        end += 1;
    }

    // If following pages are not enough, add more preceding pages if possible
    if following.len() < total_required {
        let additional_required = total_required - following.len();
        start = if x > additional_required + 3 {
            x - (additional_required + 3)
        } else {
            1
        };
        preceding.clear();
        while start < x {
            preceding.push(start);
            start += 1;
        }
    }

    vec![preceding, following]
}

#[cfg(test)]
mod x_surrounding_pages_tests {
    use super::*;

    #[test]
    fn test_middle_range() {
        let pages = x_surrounding_pages(5, 10);
        assert_eq!(pages, vec![vec![2, 3, 4], vec![6, 7, 8]]);
    }

    #[test]
    fn test_near_start() {
        let pages = x_surrounding_pages(2, 10);
        assert_eq!(pages, vec![vec![1], vec![3, 4, 5, 6, 7]]);
    }

    #[test]
    fn test_near_end() {
        let pages = x_surrounding_pages(9, 10);
        assert_eq!(pages, vec![vec![4, 5, 6, 7, 8], vec![10]]);
    }

    #[test]
    fn test_small_range() {
        let pages = x_surrounding_pages(2, 4);
        assert_eq!(pages, vec![vec![1], vec![3, 4]]);
    }

    #[test]
    fn test_boundary_conditions() {
        let pages_at_start = x_surrounding_pages(1, 10);
        assert_eq!(pages_at_start, vec![vec![], vec![2, 3, 4, 5, 6, 7]]);

        let pages_at_end = x_surrounding_pages(10, 10);
        assert_eq!(pages_at_end, vec![vec![4, 5, 6, 7, 8, 9], vec![]]);
    }

    #[test]
    fn test_x_equals_l() {
        let pages = x_surrounding_pages(5, 5);
        assert_eq!(pages, vec![vec![1, 2, 3, 4], vec![]]);
    }
}

pub fn get_ranges(vec_len: usize, range_size: usize) -> Vec<[usize; 2]> {
    let mut ranges = Vec::new();
    let mut start = 0;

    while start < vec_len {
        let end = std::cmp::min(start + range_size, vec_len);
        ranges.push([start, end]);
        start += range_size;
    }

    ranges
}

#[cfg(test)]
mod get_ranges_tests {
    use super::*;

    #[test]
    fn test_exact_divisible_range() {
        let vec_len = 20;
        let range_size = 10;
        let expected = vec![[0, 10], [10, 20]];
        assert_eq!(get_ranges(vec_len, range_size), expected);
    }

    #[test]
    fn test_not_exact_divisible_range() {
        let vec_len = 25;
        let range_size = 10;
        let expected = vec![[0, 10], [10, 20], [20, 25]];
        assert_eq!(get_ranges(vec_len, range_size), expected);
    }

    #[test]
    fn test_empty_vector() {
        let vec_len = 0;
        let range_size = 10;
        let expected: Vec<[usize; 2]> = Vec::new();
        assert_eq!(get_ranges(vec_len, range_size), expected);
    }

    #[test]
    fn test_range_size_larger_than_vector() {
        let vec_len = 5;
        let range_size = 10;
        let expected = vec![[0, 5]];
        assert_eq!(get_ranges(vec_len, range_size), expected);
    }

    #[test]
    fn test_range_size_one() {
        let vec_len = 3;
        let range_size = 1;
        let expected = vec![[0, 1], [1, 2], [2, 3]];
        assert_eq!(get_ranges(vec_len, range_size), expected);
    }
}

#[cfg(test)]
mod format_duration_tests {
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
