use chrono::{DateTime, Duration, Utc};
use leptos::*;
use rust_decimal::Decimal;

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

pub fn non_canonical_wrapper(el: HtmlElement<html::AnyElement>) -> HtmlElement<html::AnyElement> {
    html::span()
        .attr("class", "non-canonical opacity-50")
        .child(el)
        .into()
}

pub fn canonical_wrapper(el: HtmlElement<html::AnyElement>) -> HtmlElement<html::AnyElement> {
    html::span().attr("class", "canonical").child(el).into()
}

pub fn convert_array_to_span(
    els: Vec<HtmlElement<html::AnyElement>>,
) -> HtmlElement<html::AnyElement> {
    html::span()
        .attr("class", "flex items-center")
        .child(els)
        .into()
}

pub fn convert_to_status_bubble(status: Option<String>) -> HtmlElement<html::AnyElement> {
    let (color, title) = match status {
        Some(s) => (String::from("bg-status-failed"), s.to_string()),
        None => (String::from("bg-status-success"), String::new()),
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
mod tests {
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

pub fn convert_to_pill(data: String, pill_variant: PillVariant) -> HtmlElement<html::AnyElement> {
    wrap_in_pill(html::span().child(data).into(), pill_variant)
}

pub fn wrap_in_pill(any: HtmlElement<html::AnyElement>, pill_variant: PillVariant) -> HtmlElement<html::AnyElement> {
    let value_class_str_base = "text-white p-0.5 text-sm flex justify-center items-center w-fit";
    let pill_class_str_base = format!("{} {}", value_class_str_base, "px-2 rounded-full");

    let pill_class_str = format!(
        "{} {}",
        pill_class_str_base.to_owned(),
        pill_variant_to_style_str(pill_variant)
    );
    view! {
        <span class=pill_class_str>
            {any}
        </span>
    }.into()
}

pub fn data_placeholder() -> HtmlElement<html::AnyElement> {
    html::span()
        .attr(
            "class",
            "block animate-pulse h-7 w-full min-w-40 rounded-full bg-slate-200",
        )
        .into()
}

pub fn decorate_with_currency_tag(data: String, currency_tag: String) -> HtmlElement<html::AnyElement> {
    view! {
        <span class="whitespace-nowrap">
            {data} <span class="ml-1 uppercase font-light text-inherit/50">{currency_tag}</span>
        </span> 
    }.into()
}

pub fn string_to_f64(str: &str) -> Option<f64> {
    let float_val: Result<f64, _> = str.parse();
    match float_val {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

const MINA_SCALE: u32 = 9;

pub fn nanomina_to_mina(num: f64) -> String {
    let rounded = unsafe { num.to_int_unchecked::<u64>() };
    let mut dec = Decimal::from(rounded);
    dec.set_scale(MINA_SCALE).unwrap();
    let mut dec_str = dec.to_string();
    if dec_str.contains('.') {
        while dec_str.ends_with('0') {
            dec_str.pop();
        }
        if dec_str.ends_with('.') {
            dec_str.pop();
        }
    }
    dec_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_value() {
        assert_eq!(nanomina_to_mina(0.0), "0");
    }

    #[test]
    fn test_exact_value() {
        assert_eq!(nanomina_to_mina(123456789.0), "0.123456789");
    }

    #[test]
    fn test_rounding() {
        // This test assumes that the function should round down, based on the implementation
        assert_eq!(nanomina_to_mina(123456789.123), "0.123456789");
    }

    #[test]
    fn test_large_number() {
        assert_eq!(
            nanomina_to_mina(1_000_000_111_111_111.0),
            "1000000.111111111"
        );
    }

    #[test]
    fn test_larger_number() {
        // document lack of precision in the conversion from f64 to u64
        assert_eq!(
            nanomina_to_mina(10_000_000_111_111_111.0),
            "10000000.111111112"
        );
    }

    #[test]
    fn test_even_larger_number() {
        // document lack of precision in the conversion from f64 to u64
        assert_eq!(
            nanomina_to_mina(100_000_000_111_111_111.0),
            "100000000.111111104"
        );
    }

    #[test]
    fn test_small_fractional_value() {
        assert_eq!(nanomina_to_mina(1.0), "0.000000001");
    }

    #[test]
    fn test_boundary_value() {
        // Testing value just below a rounding boundary
        assert_eq!(nanomina_to_mina(123456788.999999999), "0.123456789");
    }
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

pub fn pill_variant_to_style_str(pill_variant: PillVariant) -> String {
    match pill_variant {
        PillVariant::Green => "bg-pill-green".to_string(),
        PillVariant::Blue => "bg-pill-blue".to_string(),
        PillVariant::Orange => "bg-granola-orange".to_string(),
        PillVariant::Grey => "bg-slate-400".to_string(),
    }
}

pub fn get_subset<T>(
    items: &Vec<Option<T>>,
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
) -> Pagination {
    Pagination {
        current_page,
        records_per_page,
        total_records,
        set_current_page,
    }
}
