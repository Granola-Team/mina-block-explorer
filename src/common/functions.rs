use super::models::*;
use crate::{
    common::{components::CopyToClipboard, constants::*},
    icons::HelpIcon,
};
use chrono::{DateTime, Duration, LocalResult, TimeZone, Utc};
use leptos::*;
use rand::{
    distributions::{Alphanumeric, Uniform},
    prelude::Distribution,
    Rng,
};
use rust_decimal::prelude::*;
use std::iter;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Intl::NumberFormat, *};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = language)]
    static LANGUAGE: String;
}

#[wasm_bindgen]
pub fn get_browser_locale() -> String {
    LANGUAGE.clone()
}

fn format_number_helper(number: &str, max_significant_digits: Option<u32>) -> String {
    let locale_array = Array::new();
    locale_array.push(&JsValue::from_str(&get_browser_locale()));

    let options = Object::new();
    if let Some(digits) = max_significant_digits {
        // Safely set the maximumSignificantDigits only if within a valid range:
        // See: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat
        if (1..=21).contains(&digits) {
            #[allow(unused_unsafe)]
            unsafe {
                Reflect::set(
                    &options,
                    &JsValue::from_str("minimumFractionDigits"),
                    &JsValue::from_f64(0f64),
                )
                .unwrap();
                Reflect::set(
                    &options,
                    &JsValue::from_str("maximumFractionDigits"),
                    &JsValue::from_f64(digits as f64),
                )
                .unwrap();
            };
        }
    }

    let num_format = NumberFormat::new(&locale_array, &options);
    let format = num_format.format();
    let this = JsValue::NULL; // `this` context for the function, not needed here

    match format.call1(&this, &JsValue::from_str(number)) {
        Ok(result) => result.as_string().unwrap_or_else(|| number.to_string()),
        Err(_) => number.to_string(),
    }
}

#[wasm_bindgen]
pub fn format_number(number: String) -> String {
    format_number_helper(&number, None) // No significant digits limit
}

#[wasm_bindgen]
pub fn format_mina(number: String) -> String {
    format_number_helper(&number, Some(9)) // Use 9 significant digits
}

pub fn format_metadata<F>(meta: &TableMetadata, format_number: F) -> String
where
    F: Fn(String) -> String,
{
    let displayed = format_number(meta.displayed_records.to_string());
    let total = match meta.total_records {
        Some(records) => format_number(records.to_string()),
        None => String::from("?"),
    };

    match meta.available_records {
        Some(available_records) => {
            let available = format_number(available_records.to_string());
            format!("{} of {} of {}", displayed, available, total)
        }
        None => {
            if meta.displayed_records > (TABLE_ROW_LIMIT - 1).try_into().unwrap() {
                format!("{}+ of {}", displayed, total)
            } else {
                format!("{} of {}", displayed, total)
            }
        }
    }
}

#[cfg(test)]
mod format_metadata_tests {
    use super::*;

    #[test]
    fn test_with_full_data() {
        let meta = TableMetadata {
            displayed_records: 50,
            available_records: Some(100),
            total_records: Some(200),
        };
        assert_eq!(format_metadata(&meta, |a| a), "50 of 100 of 200");
    }

    #[test]
    fn test_with_no_available_records_and_display_under_limit() {
        let meta = TableMetadata {
            displayed_records: (TABLE_ROW_LIMIT - 1) as u64,
            available_records: None,
            total_records: Some(200),
        };
        assert_eq!(
            format_metadata(&meta, |a| a),
            format!("{} of 200", TABLE_ROW_LIMIT - 1)
        );
    }

    #[test]
    fn test_with_no_available_records_and_display_over_limit() {
        let meta = TableMetadata {
            displayed_records: TABLE_ROW_LIMIT as u64,
            available_records: None,
            total_records: Some(300),
        };
        assert_eq!(
            format_metadata(&meta, |a| a),
            format!("{}+ of 300", TABLE_ROW_LIMIT)
        );
    }

    #[test]
    fn test_with_unknown_total_records() {
        let meta = TableMetadata {
            displayed_records: 150,
            available_records: Some(250),
            total_records: None,
        };
        assert_eq!(format_metadata(&meta, |a| a), "150 of 250 of ?");
    }

    #[test]
    fn test_all_unknown() {
        let meta = TableMetadata {
            displayed_records: 150,
            available_records: None,
            total_records: None,
        };
        assert_eq!(format_metadata(&meta, |a| a), "150+ of ?");
    }
}

fn split_f64(number: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = number.split('.').collect();

    if parts.len() != 2 {
        return Err("Number does not have a fractional part.".into());
    }

    let integer_part = parts[0].to_string();
    let fractional_part = parts[1].trim_end_matches('0').to_string();

    Ok((integer_part, fractional_part))
}

pub fn format_number_for_html(number: &str, max_digits_before_decimal: usize) -> String {
    match split_f64(number) {
        Ok((integer_part, fractional_part)) => {
            let padded_integer_part = format!(
                "{:>width$}",
                integer_part,
                width = max_digits_before_decimal
            );
            let padded_fractional_part = format!("{:<9}", fractional_part);
            format!("{}.{}", padded_integer_part, padded_fractional_part)
        }
        _ => {
            format!("{}", number)
        }
    }
}

#[cfg(test)]
mod format_number_for_html_tests {
    use super::*;

    #[test]
    fn test_format_number_with_decimal() {
        let number = "123.456";
        let max_digits_before_decimal = 4;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            " 123.456      "
        );
    }

    #[test]
    fn test_number_shorter_than_padding() {
        let number = "78.123456789";
        let max_digits_before_decimal = 5;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            "   78.123456789"
        );
    }

    #[test]
    fn test_number_longer_than_padding() {
        let number = "123456.123";
        let max_digits_before_decimal = 4;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            "123456.123      "
        );
    }

    #[test]
    fn test_negative_number() {
        let number = "-123.456";
        let max_digits_before_decimal = 5;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            " -123.456      "
        );
    }

    #[test]
    fn test_very_small_fractional() {
        let number = "123.000000001";
        let max_digits_before_decimal = 4;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            " 123.000000001"
        );
    }

    #[test]
    fn test_large_number() {
        let number = "9676018.01490734";
        let max_digits_before_decimal = 9;
        assert_eq!(
            format_number_for_html(number, max_digits_before_decimal),
            "  9676018.01490734 "
        );
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
    html::span()
        .child(data)
        .attr("class", "whitespace-pre")
        .into()
}

pub fn convert_to_title(data: String, title: String) -> HtmlElement<html::AnyElement> {
    html::span()
        .child(data)
        .attr("class", "cursor-help")
        .attr("title", title)
        .into()
}

pub fn convert_array_to_span(
    els: Vec<HtmlElement<html::AnyElement>>,
) -> HtmlElement<html::AnyElement> {
    html::span()
        .attr("class", "flex items-center")
        .child(els)
        .into()
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
            "loading-placeholder block animate-pulse h-7 w-full rounded-full bg-slate-200",
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

pub fn convert_to_link(data: String, href: String) -> HtmlElement<html::AnyElement> {
    view! {
        <span class="w-full text-ellipsis overflow-hidden">
            <CopyToClipboard>
                <a href=href class=LINK_HOVER_STATE>
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

fn format_duration(duration: &Duration) -> String {
    let seconds = duration.num_seconds();

    let years = seconds / (365 * 24 * 60 * 60);
    let months = (seconds % (365 * 24 * 60 * 60)) / (30 * 24 * 60 * 60);
    let days = (seconds % (30 * 24 * 60 * 60)) / (24 * 60 * 60);
    let hours = (seconds % (24 * 60 * 60)) / (60 * 60);
    let minutes = (seconds % (60 * 60)) / 60;

    let parts = [
        (years, "yr"),
        (months, "mo"),
        (days, "d"),
        (hours, "h"),
        (minutes, "min"),
    ];

    let filtered_parts: Vec<_> = parts
        .iter()
        .filter(|&&(value, _)| value > 0)
        .map(|&(value, name)| format!("{} {}", value, name))
        .collect();

    let num_parts = filtered_parts.len();

    match num_parts {
        0 => "just now".to_string(),
        1..=2 => filtered_parts.join(" "),
        _ => filtered_parts[..2].join(" "),
    }
}

pub fn print_time_since(timestamp: &str) -> String {
    let past_time = match timestamp.parse::<DateTime<Utc>>() {
        Ok(time) => time,
        Err(_e) => return "Unknown".to_string(),
    };

    let now = Utc::now();
    let duration_since = now.signed_duration_since(past_time);
    format_duration(&duration_since)
}

#[cfg(test)]
mod format_duration_tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn get_past_time(duration: Duration) -> DateTime<Utc> {
        Utc::now() - duration
    }

    #[test]
    fn test_one_minute_ago() {
        let past_time = get_past_time(Duration::minutes(1));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "1 min");
    }

    #[test]
    fn test_multiple_minutes_ago() {
        let past_time = get_past_time(Duration::minutes(20));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "20 min");
    }

    #[test]
    fn test_one_hour_ago() {
        let past_time = get_past_time(Duration::hours(1));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "1 h");
    }

    #[test]
    fn test_multiple_hours_ago() {
        let past_time = get_past_time(Duration::hours(3));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "3 h");
    }

    #[test]
    fn test_one_day_ago() {
        let past_time = get_past_time(Duration::days(1));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "1 d");
    }

    #[test]
    fn test_multiple_days_ago() {
        let past_time = get_past_time(Duration::days(5));
        assert_eq!(print_time_since(&past_time.to_rfc3339()), "5 d");
    }

    #[test]
    fn test_just_now() {
        let now = Utc::now();
        assert_eq!(print_time_since(&now.to_rfc3339()), "just now");
    }

    #[test]
    fn test_invalid_timestamp() {
        assert_eq!(print_time_since("not a real timestamp"), "Unknown");
    }

    #[test]
    fn test_complex_duration() {
        let duration = Duration::days(410) + Duration::hours(25) + Duration::minutes(61);
        assert_eq!(format_duration(&duration), "1 yr 1 mo");
    }

    #[test]
    fn test_mix_days_hours_minutes() {
        let duration = Duration::days(2) + Duration::hours(3) + Duration::minutes(45);
        assert_eq!(format_duration(&duration), "2 d 3 h");
    }
}

pub fn pill_variant_to_style_str(pill_variant: ColorVariant) -> String {
    match pill_variant {
        ColorVariant::Green => "bg-green".to_string(),
        ColorVariant::Blue => "bg-blue".to_string(),
        ColorVariant::Grey => "bg-slate-400".to_string(),
        ColorVariant::Transparent => "bg-transparent".to_string(),
        ColorVariant::DarkBlue => "bg-dark-blue".to_string(),
        ColorVariant::Orange => "bg-amber-600".to_string(),
    }
}
