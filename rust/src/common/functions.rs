use super::models::*;
use crate::{
    common::{components::CopyToClipboard, constants::*},
    icons::HelpIcon,
};
use chrono::{DateTime, Duration, Utc};
use leptos::*;
use rust_decimal::prelude::*;
use serde_json::Value;
use wasm_bindgen::{JsValue, prelude::*};
use web_sys::js_sys::{Date, Intl::NumberFormat, Object, Reflect, *};

#[wasm_bindgen]
#[allow(non_snake_case)]
extern "C" {
    #[wasm_bindgen(js_namespace = navigator, js_name = language)]
    #[wasm_bindgen(thread_local_v2)]
    static LANGUAGE: String;
}

#[wasm_bindgen]
pub fn get_browser_locale() -> String {
    LANGUAGE.with(String::clone)
}

#[wasm_bindgen]
pub fn convert_to_local_timezone_formatted(utc_date_str: &str) -> String {
    // Create a JS Date object from the UTC date string
    let date = Date::new(&JsValue::from_str(utc_date_str));

    // Define options for the toLocaleString method to get YYYY-MM-DD HH:mm:ss
    // format
    let options = Object::new();
    Reflect::set(
        &options,
        &JsValue::from_str("year"),
        &JsValue::from_str("numeric"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("month"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("day"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("hour"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("minute"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();
    Reflect::set(
        &options,
        &JsValue::from_str("second"),
        &JsValue::from_str("2-digit"),
    )
    .unwrap();

    // Use toLocaleString with the options to get the formatted string
    date.to_locale_string(&get_browser_locale(), &options)
        .into()
}

pub fn format_number_helper(number: &str, max_significant_digits: Option<u32>) -> String {
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
                    &JsValue::from_f64(1f64),
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
            if meta.displayed_records > (TABLE_ROW_LIMIT - 1) {
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
            displayed_records: TABLE_ROW_LIMIT - 1,
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
            displayed_records: TABLE_ROW_LIMIT,
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

fn split_number(number: &str) -> Result<(char, Vec<&str>), String> {
    let delim: char;
    if number.contains('.') && number.contains(',') {
        if number.rfind('.') > number.rfind(',') {
            delim = '.';
        } else {
            delim = ',';
        }
    } else if number.contains('.') {
        delim = '.';
    } else if number.contains(',') {
        delim = ','
    } else {
        return Err("Number does not have a valid decimal separator.".into());
    }

    Ok((delim, number.split(delim).collect()))
}

#[cfg(test)]
mod split_number_tests {
    use super::split_number;

    #[test]
    fn test_dot_separator() {
        let number = "123.456"; // English format
        let result = split_number(number);
        assert_eq!(result, Ok(('.', vec!["123", "456"])));
    }

    #[test]
    fn test_comma_separator() {
        let number = "123,456"; // French/German format
        let result = split_number(number);
        assert_eq!(result, Ok((',', vec!["123", "456"])));
    }

    #[test]
    fn test_both_separators_dot_last() {
        let number = "1,234.56"; // English format with thousands separator
        let result = split_number(number);
        assert_eq!(result, Ok(('.', vec!["1,234", "56"])));
    }

    #[test]
    fn test_both_separators_comma_last() {
        let number = "1.234,56"; // German format with thousands separator
        let result = split_number(number);
        assert_eq!(result, Ok((',', vec!["1.234", "56"])));
    }

    #[test]
    fn test_no_separator() {
        let number = "123456"; // No decimal separator
        let result = split_number(number);
        assert!(result.is_err());
        assert_eq!(
            result,
            Err("Number does not have a valid decimal separator.".into())
        );
    }

    #[test]
    fn test_only_integer_part() {
        let number = "123."; // Only integer part
        let result = split_number(number);
        assert_eq!(result, Ok(('.', vec!["123", ""])));
    }

    #[test]
    fn test_only_fractional_part() {
        let number = ".456"; // Only fractional part
        let result = split_number(number);
        assert_eq!(result, Ok(('.', vec!["", "456"])));
    }

    #[test]
    fn test_trailing_zeros_dot() {
        let number = "123.000"; // English format with trailing zeros
        let result = split_number(number);
        assert_eq!(result, Ok(('.', vec!["123", "000"])));
    }

    #[test]
    fn test_trailing_zeros_comma() {
        let number = "123,000"; // French/German format with trailing zeros
        let result = split_number(number);
        assert_eq!(result, Ok((',', vec!["123", "000"])));
    }

    #[test]
    fn test_french_format() {
        let number = "1 234,56"; // French format with space as thousands separator
        let result = split_number(number);
        assert_eq!(result, Ok((',', vec!["1 234", "56"])));
    }

    #[test]
    fn test_german_format() {
        let number = "1.234,56"; // German format with dot as thousands separator
        let result = split_number(number);
        assert_eq!(result, Ok((',', vec!["1.234", "56"])));
    }
}

fn split_f64(number: &str) -> Result<(char, String, String), String> {
    if split_number(number).is_err() {
        return Err("Unable to parse number.".into());
    }

    let (delim, parts) = split_number(number).unwrap();

    if parts.len() != 2 {
        return Err("Number does not have a fractional part.".into());
    }

    let integer_part = parts[0].to_string();
    let mut fractional_part = parts[1].trim_end_matches('0').to_string();
    if fractional_part.is_empty() {
        fractional_part = "0".to_string();
    }

    Ok((delim, integer_part, fractional_part))
}

pub fn format_number_for_html(number: &str, max_digits_before_decimal: usize) -> String {
    match split_f64(number) {
        Ok((delim, integer_part, fractional_part)) => {
            let padded_integer_part = format!(
                "{:>width$}",
                integer_part,
                width = max_digits_before_decimal
            );
            let padded_fractional_part = format!("{:<9}", fractional_part);
            format!("{}{}{}", padded_integer_part, delim, padded_fractional_part)
        }
        _ => number.to_string(),
    }
}

#[cfg(test)]
mod format_number_for_html_tests {
    use super::format_number_for_html;

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

pub fn convert_to_linkable_address(username: &str, address: &str) -> HtmlElement<html::AnyElement> {
    convert_array_to_span(vec![
        convert_to_link(
            username.to_string(),
            format!("/addresses/accounts/{}/spotlight", address),
        ),
        convert_to_copy_link(
            address.to_string(),
            format!("/addresses/accounts/{}/spotlight", address),
        )
        .attr("class", "text-xs text-slate-400"),
    ])
    .attr("class", "w-full text-ellipsis overflow-hidden")
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

pub fn convert_to_canonical(data: String, canonical: bool) -> HtmlElement<html::AnyElement> {
    html::span()
        .attr("class", "flex justify-between items-center w-full")
        .child(vec![
            html::span().attr(
                "class",
                format!(
                    "{} w-3 h-3 ml-2 rounded-[50%]",
                    if canonical {
                        "canonical bg-status-success"
                    } else {
                        "non-canonical bg-status-failed"
                    },
                ),
            ),
            html::span().child(data),
        ])
        .into()
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
                view! { "Free" }.into_view()
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
            <HelpIcon width=15 />
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

pub fn nanomina_to_mina_i64(num: i64) -> String {
    let abs_num = num.unsigned_abs();
    let formatted = nanomina_to_mina(abs_num);
    if num.is_negative() {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

pub fn convert_to_copy_link(data: String, href: String) -> HtmlElement<html::AnyElement> {
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

pub fn convert_to_link(data: String, href: String) -> HtmlElement<html::AnyElement> {
    view! {
        <span class="w-full text-ellipsis overflow-hidden">
            <a href=href class=LINK_HOVER_STATE>
                {data}
            </a>
        </span>
    }
    .into()
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

pub fn round_to_two_decimals(value: f64) -> Option<String> {
    if !value.is_finite() {
        return None;
    }
    // Round to 2 decimal places
    let rounded = (value * 100.0).round() / 100.0;
    Some(format!("{:.2}", rounded))
}

#[cfg(test)]
mod round_to_two_decimals_tests {
    use super::round_to_two_decimals;

    #[test]
    fn test_round_to_two_decimals() {
        // Test normal cases
        assert_eq!(round_to_two_decimals(12.3456), Some("12.35".to_string()));
        assert_eq!(round_to_two_decimals(12.3444), Some("12.34".to_string()));
        assert_eq!(round_to_two_decimals(0.0), Some("0.00".to_string()));
        assert_eq!(round_to_two_decimals(99.999), Some("100.00".to_string()));

        // Test edge cases
        assert_eq!(round_to_two_decimals(-12.3456), Some("-12.35".to_string()));
        assert_eq!(round_to_two_decimals(0.001), Some("0.00".to_string()));
        assert_eq!(round_to_two_decimals(0.005), Some("0.01".to_string()));

        // Test non-finite cases
        assert_eq!(round_to_two_decimals(f64::INFINITY), None);
        assert_eq!(round_to_two_decimals(f64::NEG_INFINITY), None);
        assert_eq!(round_to_two_decimals(f64::NAN), None);
    }
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
        ColorVariant::DarkGreen => "bg-green-dark".to_string(),
        ColorVariant::Blue => "bg-blue".to_string(),
        ColorVariant::DarkBlue => "bg-blue-dark".to_string(),
        ColorVariant::Grey => "bg-slate-400".to_string(),
        ColorVariant::DarkGrey => "bg-slate-600".to_string(),
        ColorVariant::Orange => "bg-amber-600".to_string(),
    }
}

pub fn normalize_number_format(number: &str) -> Result<String, String> {
    let delim: char;

    // Determine the decimal separator
    if number.contains('.') && number.contains(',') {
        if number.rfind('.') > number.rfind(',') {
            delim = '.';
        } else {
            delim = ',';
        }
    } else if number.contains('.') {
        delim = '.';
    } else if number.contains(',') {
        delim = ',';
    } else {
        return Err("Number does not have a valid decimal separator.".into());
    }

    // Split into LHS and RHS based on the determined decimal separator
    let parts: Vec<&str> = number.split(delim).collect();
    if parts.len() != 2 {
        return Err("Invalid number format.".into());
    }

    // Clean up LHS (remove thousands separators like spaces, dots, or commas)
    let mut lhs: String = parts[0].replace([',', '.', ' '], "");

    // If there's no integer part, replace with "0"
    if lhs.is_empty() {
        lhs = "0".to_string();
    }

    // Clean up RHS (remove any non-digit characters such as commas or dots)
    let rhs: String = parts[1].replace([',', '.', ' '], "");

    // Return the normalized number with a single dot separating LHS and RHS
    Ok(format!("{}.{}", lhs, rhs))
}

#[cfg(test)]
mod normalize_number_format_tests {
    use super::normalize_number_format;

    #[test]
    fn test_dot_separator() {
        let number = "123.456"; // English format
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("123.456".to_string()));
    }

    #[test]
    fn test_comma_separator() {
        let number = "123,456"; // French/German format
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("123.456".to_string()));
    }

    #[test]
    fn test_both_separators_dot_last() {
        let number = "1,234.56"; // English format with thousands separator
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("1234.56".to_string()));
    }

    #[test]
    fn test_both_separators_comma_last() {
        let number = "1.234,56"; // German format with thousands separator
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("1234.56".to_string()));
    }

    #[test]
    fn test_no_separator() {
        let number = "123456"; // No decimal separator
        let result = normalize_number_format(number);
        assert!(result.is_err());
    }

    #[test]
    fn test_only_integer_part() {
        let number = "123."; // Only integer part
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("123.".to_string()));
    }

    #[test]
    fn test_only_fractional_part() {
        let number = ".456"; // Only fractional part
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("0.456".to_string()));
    }

    #[test]
    fn test_trailing_zeros_dot() {
        let number = "123.000"; // English format with trailing zeros
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("123.000".to_string()));
    }

    #[test]
    fn test_trailing_zeros_comma() {
        let number = "123,000"; // French/German format with trailing zeros
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("123.000".to_string()));
    }

    #[test]
    fn test_french_format() {
        let number = "1 234,56"; // French format with space as thousands separator
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("1234.56".to_string()));
    }

    #[test]
    fn test_german_format() {
        let number = "1.234,56"; // German format with dot as thousands separator
        let result = normalize_number_format(number);
        assert_eq!(result, Ok("1234.56".to_string()));
    }
}

const MAXIMUM_BLOCK_RANGE: u64 = 2000;
pub fn validate_block_height_range(
    blockheight_gte_opt: Option<u64>,
    blockheight_lte_opt: Option<u64>,
) -> Result<(), &'static str> {
    if blockheight_gte_opt.is_none() {
        return Err("Missing start block height");
    }
    if blockheight_lte_opt.is_none() {
        return Err("Missing end block height");
    }
    let blockheight_gte = blockheight_gte_opt.unwrap();
    let blockheight_lte = blockheight_lte_opt.unwrap();
    if blockheight_gte >= blockheight_lte {
        return Err("End block must be larger than start block");
    }
    if blockheight_lte - blockheight_gte > MAXIMUM_BLOCK_RANGE {
        return Err("Block range must not exceed 2000");
    }
    Ok(())
}

#[cfg(test)]
mod validate_block_height_range_tests {
    use super::*;

    #[test]
    fn test_validate_block_height_range() {
        // Case 1: Missing start block height
        assert_eq!(
            validate_block_height_range(None, Some(100)),
            Err("Missing start block height")
        );

        // Case 2: Missing end block height
        assert_eq!(
            validate_block_height_range(Some(10), None),
            Err("Missing end block height")
        );

        // Case 3: End block is not larger than start block
        assert_eq!(
            validate_block_height_range(Some(100), Some(50)),
            Err("End block must be larger than start block")
        );

        // Case 3: Block range is too large
        assert_eq!(
            validate_block_height_range(Some(0), Some(2001)),
            Err("Block range must not exceed 2000"),
        );

        // Case 4: Valid range
        assert_eq!(validate_block_height_range(Some(50), Some(100)), Ok(()));
    }
}

#[wasm_bindgen(
    inline_js = "export function get_unix_timestamp() { return Math.floor(Date.now() / 1000); }"
)]
extern "C" {
    pub fn get_unix_timestamp() -> f64;
}

pub fn get_button_style_variation(style_variant: &ButtonStyleVariant) -> &str {
    match style_variant {
        ButtonStyleVariant::Primary => "text-white bg-granola-orange",
        ButtonStyleVariant::Secondary => "text-granola-orange bg-white",
        ButtonStyleVariant::Tertiary => {
            "text-slate-500 bg-white border-slate-500 disabled:text-slate-300 disabled:border-slate-300"
        }
    }
}

pub fn format_json_array_pretty(vec: Vec<Option<String>>) -> Result<String, serde_json::Error> {
    // Convert Vec<Option<String>> into a Vec<Value> where None becomes null
    let json_array: Vec<Value> = vec
        .into_iter()
        .map(|opt| match opt {
            Some(s) => Value::String(s),
            None => Value::Null,
        })
        .collect();

    // Wrap it as a serde_json::Value::Array
    let json_value = Value::Array(json_array);

    // Serialize to pretty-printed string
    let pretty = serde_json::to_string_pretty(&json_value)?;
    Ok(pretty)
}
