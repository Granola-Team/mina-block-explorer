use super::models::*;
use crate::{common::components::CopyToClipboard, icons::HelpIcon};
use chrono::{DateTime, Duration, LocalResult, TimeZone, Utc};
use leptos::*;
use rand::{
    distributions::{Alphanumeric, Uniform},
    prelude::Distribution,
    Rng,
};
use rust_decimal::prelude::*;
use std::iter;

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

pub fn convert_array_to_span(
    els: Vec<HtmlElement<html::AnyElement>>,
) -> HtmlElement<html::AnyElement> {
    html::span()
        .attr("class", "flex items-center")
        .child(els)
        .into()
}

pub fn convert_to_status_bubble(
    canonical_status: Option<bool>,
    status_msg: Option<String>,
) -> HtmlElement<html::AnyElement> {
    let (color, title) = match (canonical_status, status_msg) {
        (Some(false), Some(s)) => (String::from("bg-status-failed"), s.to_string()),
        (Some(false), None) => (String::from("bg-status-failed"), "Unknown".to_string()),
        (Some(true), _) => (String::from("bg-status-success"), String::new()),
        (None, _) => (
            String::from("bg-status-unknown"),
            "Canonical Status Unknown".to_string(),
        ),
    };
    html::span()
        .attr(
            "class",
            format!(
                "block h-3 w-3 rounded-full mr-1 {} {}",
                color,
                if !title.is_empty() {
                    String::from("cursor-help")
                } else {
                    String::new()
                }
            ),
        )
        .attr("title", split_and_title_case(&title, '_').join(" "))
        .into()
}

pub fn to_title_case(s: &str) -> String {
    s.char_indices()
        .map(|(i, c)| {
            if i == 0 || s[i - 1..i].contains(' ') {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

pub fn split_and_title_case(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(to_title_case).collect()
}

#[cfg(test)]
mod titlecase_tests {
    use super::{split_and_title_case, to_title_case};

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello"), "Hello");
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("hello_world"), "Hello_world");
        assert_eq!(
            to_title_case("hello world_from rust"),
            "Hello World_from Rust"
        );
    }

    #[test]
    fn test_split_and_title_case() {
        assert_eq!(
            split_and_title_case("hello_world_from_rust", '_'),
            vec!["Hello", "World", "From", "Rust"]
        );
        assert_eq!(
            split_and_title_case("one_two_three", '_'),
            vec!["One", "Two", "Three"]
        );
        assert_eq!(split_and_title_case("a-b-c", '-'), vec!["A", "B", "C"]);
    }

    #[test]
    fn test_split_and_title_case_with_join() {
        let input = "hello_world_from_rust";
        let delimiter = '_';
        let title_cased = split_and_title_case(input, delimiter);

        // Test the joined string
        let result = title_cased.join(" ");
        assert_eq!(result, "Hello World From Rust");
    }
}

pub fn convert_to_pill(data: String, pill_variant: ColorVariant) -> HtmlElement<html::AnyElement> {
    wrap_in_pill(html::span().child(data).into(), pill_variant)
}

pub fn wrap_in_pill(
    any: HtmlElement<html::AnyElement>,
    pill_variant: ColorVariant,
) -> HtmlElement<html::AnyElement> {
    let value_class_str_base = "text-white p-0.5 flex justify-center items-center w-fit";
    let pill_class_str_base = format!("{} {}", value_class_str_base, "px-2 rounded-full");

    let pill_class_str = format!(
        "{} {}",
        pill_class_str_base.to_owned(),
        pill_variant_to_style_str(pill_variant)
    );
    view! { <span class=pill_class_str>{any}</span> }.into()
}

pub fn data_placeholder() -> HtmlElement<html::AnyElement> {
    html::span()
        .attr(
            "class",
            "loading-placeholder block animate-pulse h-7 w-full min-w-40 rounded-full bg-slate-200",
        )
        .into()
}

pub fn decorate_with_mina_tag(data: String) -> HtmlElement<html::AnyElement> {
    decorate_with_currency_tag(data, "MINA".to_string())
}

pub fn decorate_with_currency_tag(
    data: String,
    currency_tag: String,
) -> HtmlElement<html::AnyElement> {
    #![allow(unused_braces)]
    view! {
        <span class="whitespace-nowrap">
            {if data != "0" {
                view! {
                    {data}
                    <span class="ml-1 uppercase font-light text-inherit/50">{currency_tag}</span>
                }
                    .into_view()
            } else {
                view! { {data} }.into_view()
            }}

        </span>
    }
    .into()
}

pub fn convert_to_tooltip(tooltip: String) -> HtmlElement<html::AnyElement> {
    view! {
        <span
            title=tooltip
            class="tooltip text-slate-600 font-sans text-xs m-0.5 p-1 hover:bg-slate-200 rounded-full cursor-help"
        >
            <HelpIcon width=15/>
        </span>
    }.into()
}

const MINA_SCALE: u32 = 9;

pub fn nanomina_str_to_mina(n_str: &str) -> String {
    let dec = Decimal::from_str(n_str).unwrap();
    nanomina_to_mina(dec.to_u64().unwrap())
}

pub fn nanomina_to_mina(num: u64) -> String {
    let mut dec = Decimal::from(num);
    dec.set_scale(MINA_SCALE).unwrap();
    let num_str = dec.to_string();
    format_mina(num_str)
}

pub fn format_mina(num_str: String) -> String {
    let parts: Vec<&str> = num_str.split('.').collect();
    let mut integral_part = parts[0].to_string();
    let decimal_part = parts.get(1).unwrap_or(&"").to_string();

    if integral_part.len() > 3 {
        let mut index = integral_part.len() - 3;
        while index > 0 {
            integral_part.insert(index, ',');
            if index > 3 {
                index -= 3;
            } else {
                break;
            }
        }
    }

    let trimmed_decimal_part = decimal_part.trim_end_matches('0');

    if !trimmed_decimal_part.is_empty() {
        format!("{}.{}", integral_part, trimmed_decimal_part)
    } else {
        integral_part
    }
}

#[cfg(test)]
mod nanomina_tests {
    use super::*;

    #[test]
    fn test_zero_value() {
        assert_eq!(nanomina_to_mina(0), "0");
    }

    #[test]
    fn test_exact_value() {
        assert_eq!(nanomina_to_mina(123_456_789), "0.123456789");
    }

    #[test]
    fn test_large_number() {
        assert_eq!(
            nanomina_to_mina(5_000_000_000_111_111_111),
            "5,000,000,000.111111111"
        );
    }

    #[test]
    fn test_small_integer_value() {
        assert_eq!(nanomina_to_mina(1), "0.000000001");
    }

    #[test]
    fn test_boundary_value() {
        assert_eq!(nanomina_to_mina(999_999_999), "0.999999999");
    }
}

pub fn convert_to_link(data: String, href: String) -> HtmlElement<html::AnyElement> {
    view! {
        <span class="w-full text-ellipsis overflow-hidden">
            <CopyToClipboard>
                <a href=href class="hover:text-granola-orange hover:underline hover:decoration-2">
                    {convert_to_ellipsis(data)}
                </a>
            </CopyToClipboard>
        </span>
    }
    .into()
}

pub fn generate_random_string(len: usize) -> String {
    iter::repeat(())
        .map(|()| rand::thread_rng().sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

#[cfg(test)]
mod generate_random_string_tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_random_string_length() {
        let length = 10;
        let random_string = generate_random_string(length);
        assert_eq!(random_string.len(), length);
    }

    #[test]
    fn test_non_zero_length_returns_non_empty_string() {
        let length = 1;
        let random_string = generate_random_string(length);
        assert!(!random_string.is_empty());
    }

    #[test]
    fn test_zero_length_returns_empty_string() {
        let length = 0;
        let random_string = generate_random_string(length);
        assert!(random_string.is_empty());
    }

    #[test]
    fn test_generated_string_is_alphanumeric() {
        let length = 100; // Use a reasonably large length to have a good sample
        let random_string = generate_random_string(length);
        assert!(random_string.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_randomness() {
        let length = 10;
        let tries = 100;
        let mut unique_strings = HashSet::new();
        for _ in 0..tries {
            let random_string = generate_random_string(length);
            unique_strings.insert(random_string);
        }
        // This test may fail occasionally; it's a probabilistic approach to testing
        // randomness
        assert!(unique_strings.len() > 1, "Generated strings are not random");
    }
}

pub fn generate_random_mina_price() -> f64 {
    let mut rng = rand::thread_rng();
    let balance_dist = Uniform::from(0.0..=1000.0);
    let balance = balance_dist.sample(&mut rng);
    let formatted_balance = format!("{:.9}", balance);
    formatted_balance.parse::<f64>().unwrap()
}

#[cfg(test)]
mod generate_random_mina_price_tests {
    use super::*;

    #[test]
    fn test_generate_random_mina_price_range() {
        let price = generate_random_mina_price();
        // Check that the price is within the expected range
        assert!((0.0..=1000.0).contains(&price));
    }
}

pub fn generate_random_datetime_within_days(days_before_today: i64) -> DateTime<Utc> {
    let mut rng = rand::thread_rng();

    // Calculate today's date and the start date (today - x days)
    let today = Utc::now();
    let start = today - Duration::days(days_before_today);

    // Convert start and today to timestamps (seconds since the epoch)
    let start_timestamp = start.timestamp();
    let end_timestamp = today.timestamp();

    // Generate a random timestamp between start and today
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);

    // Convert the random timestamp back to DateTime<Utc>
    match Utc.timestamp_opt(random_timestamp, 0) {
        LocalResult::Single(datetime) => datetime,
        _ => panic!("Invalid timestamp generated"),
    }
}

#[cfg(test)]
mod generate_random_datetime_within_days_tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn random_datetime_is_within_range() {
        let days_before_today = 30;
        let generated_date = generate_random_datetime_within_days(days_before_today);
        let today = Utc::now();
        let start_date = today - Duration::days(days_before_today);

        // Check that the generated date is not earlier than start_date and not later
        // than today
        assert!(
            generated_date >= start_date && generated_date <= today,
            "Generated datetime is not within the expected range."
        );
    }

    #[test]
    fn random_datetime_today() {
        // Generate a date for "0" days before today, which should effectively be today
        let days_before_today = 0;
        let generated_date = generate_random_datetime_within_days(days_before_today);
        let today = Utc::now();

        // Considering some small computation time, allow a minute difference
        let diff = today - generated_date;
        assert!(
            diff < Duration::minutes(1),
            "Generated datetime should be close to now."
        );
    }

    #[test]
    fn random_datetime_within_range() {
        // Ensures that the range is not empty by generating a range that is always
        // valid
        let days_before_today = 1; // Adjust this to a positive number to avoid an empty range
        let generated_date = generate_random_datetime_within_days(days_before_today);
        let today = Utc::now();
        let start_date = today - Duration::days(days_before_today);

        assert!(
            generated_date >= start_date && generated_date <= today,
            "Generated datetime is not within the expected range."
        );
    }
}

pub fn generate_base58_string(len: usize) -> String {
    const BASE58_CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..BASE58_CHARS.len());
            BASE58_CHARS[idx] as char
        })
        .collect()
}

pub fn convert_to_ellipsis(text: String) -> HtmlElement<html::AnyElement> {
    let parts_base = "overflow-hidden flex-initial";
    let (first, last) = split_str(&text);
    view! {
        <div class="w-full flex items-baseline justify-start">
            <div class=format!("{} break-all", parts_base)>{first}</div>
            <div
                style="direction: rtl;"
                class=format!(
                    r#"{} whitespace-nowrap text-ellipsis after:content-['\200E']"#,
                    parts_base,
                )
            >

                {last}
            </div>
        </div>
    }
    .into()
}

pub fn split_str(s: &str) -> (String, String) {
    let mid = s.len() / 2;
    let mut boundary = mid;

    for (idx, _) in s.char_indices() {
        if idx >= mid {
            boundary = idx;
            break;
        }
    }

    let (first_half, second_half) = s.split_at(boundary);
    (first_half.to_string(), second_half.to_string())
}

#[cfg(test)]
mod split_str_tests {
    use super::split_str;

    #[test]
    fn even_length_ascii() {
        let input = "HelloWorld";
        let (first_half, second_half) = split_str(input);
        assert_eq!(first_half, "Hello");
        assert_eq!(second_half, "World");
    }

    #[test]
    fn odd_length_ascii() {
        let input = "Hello";
        let (first_half, second_half) = split_str(input);
        assert_eq!(first_half, "He");
        assert_eq!(second_half, "llo");
    }

    #[test]
    fn single_char() {
        let input = "H";
        let (first_half, second_half) = split_str(input);
        assert_eq!(first_half, "");
        assert_eq!(second_half, "H");
    }

    #[test]
    fn two_chars() {
        let input = "Hi";
        let (first_half, second_half) = split_str(input);
        assert_eq!(first_half, "H");
        assert_eq!(second_half, "i");
    }

    #[test]
    fn empty_string() {
        let input = "";
        let (first_half, second_half) = split_str(input);
        assert_eq!(first_half, "");
        assert_eq!(second_half, "");
    }
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
        let duration = Duration::try_days(3);
        assert_eq!(format_duration(&duration.unwrap()), "3 days ago");
    }

    #[test]
    fn test_format_duration_hours() {
        let duration = Duration::try_hours(5);
        assert_eq!(format_duration(&duration.unwrap()), "5 hours ago");
    }

    #[test]
    fn test_format_duration_minutes() {
        let duration = Duration::try_minutes(45);
        assert_eq!(format_duration(&duration.unwrap()), "45 minutes ago");
    }

    #[test]
    fn test_format_duration_mix() {
        let duration = Duration::try_hours(26);
        assert_eq!(format_duration(&duration.unwrap()), "1 days ago");
    }
}

pub fn pill_variant_to_style_str(pill_variant: ColorVariant) -> String {
    match pill_variant {
        ColorVariant::Green => "bg-green".to_string(),
        ColorVariant::Blue => "bg-blue".to_string(),
        ColorVariant::Grey => "bg-slate-400".to_string(),
        ColorVariant::Transparent => "bg-transparent".to_string(),
        ColorVariant::DarkBlue => "bg-dark-blue".to_string(),
    }
}

pub fn get_subset<T>(
    items: &[Option<T>],
    records_per_page: usize,
    current_range: usize,
) -> Vec<Option<T>>
where
    T: Clone,
{
    let total_records = items.len();
    if total_records > 0 {
        let ranges = get_ranges(total_records, records_per_page);
        let range = ranges[current_range];
        items[range[0]..range[1]].to_vec()
    } else {
        vec![]
    }
}

#[cfg(test)]
mod get_subset_tests {
    use super::get_subset;

    #[derive(Debug, Clone, PartialEq)]
    struct MyStruct {
        value: i32,
    }

    #[test]
    fn test_get_subset_with_zero_length() {
        let data: Vec<Option<MyStruct>> = vec![];

        let records_per_page = 1;
        let current_range = 0;
        let result = get_subset(&data, records_per_page, current_range);

        assert_eq!(result, vec![] as Vec<Option<MyStruct>>);
    }

    #[test]
    fn test_get_subset_with_full_range() {
        let data: Vec<Option<MyStruct>> = vec![
            Some(MyStruct { value: 1 }),
            Some(MyStruct { value: 2 }),
            Some(MyStruct { value: 3 }),
            Some(MyStruct { value: 4 }),
            Some(MyStruct { value: 5 }),
        ];

        let records_per_page = data.len();
        let current_range = 0;
        let result = get_subset(&data, records_per_page, current_range);
        assert_eq!(result, data);
    }

    #[test]
    fn test_get_subset_with_partial_range() {
        let data: Vec<Option<MyStruct>> = vec![
            Some(MyStruct { value: 1 }),
            Some(MyStruct { value: 2 }),
            Some(MyStruct { value: 3 }),
            Some(MyStruct { value: 4 }),
            Some(MyStruct { value: 5 }),
        ];

        let records_per_page = 2;
        let current_range = 1;
        let result = get_subset(&data, records_per_page, current_range);
        assert_eq!(
            result,
            vec![Some(MyStruct { value: 3 }), Some(MyStruct { value: 4 })]
        );
    }
}

pub fn build_pagination(
    total_records: usize,
    records_per_page: usize,
    current_page: usize,
    set_current_page: WriteSignal<usize>,
    page_height: Option<usize>,
    scaling_function: Option<Box<dyn Fn(usize) -> usize>>,
) -> Pagination {
    let resolved_scaling_function =
        scaling_function.unwrap_or_else(|| Box::new(|_x: usize| records_per_page));

    Pagination {
        current_page,
        records_per_page: page_height
            .map(resolved_scaling_function)
            .unwrap_or(records_per_page),
        total_records,
        set_current_page,
    }
}
