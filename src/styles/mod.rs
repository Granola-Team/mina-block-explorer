pub fn get_desktop_breakout_container_styles(center_col: Option<Vec<String>>) -> Vec<String> {
    let column_template = vec![
        "10%".to_string(),
        center_col.unwrap_or(vec!["80%".to_string()]).join("_"),
        "10%".to_string()
    ];
    vec![
        "md:grid".to_string(),
        format!("md:grid-cols-[{}]",column_template.join("_")),
    ]
}

pub fn get_desktop_breakout_child_styles() -> Vec<String> {
    vec![
        "md:col-start-2".to_string(),
        "md:col-end-3".to_string()
    ]
}

pub trait ToStylesString {
    fn to_styles_string(&self) -> String;
}

// Implement ToStylesString for Vec<String>
impl ToStylesString for Vec<String> {
    fn to_styles_string(&self) -> String {
        self.join(" ")
    }
}




