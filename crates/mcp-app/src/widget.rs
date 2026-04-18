pub const HELLO_WIDGET_URI: &str = "ui://app/hello.html";

pub fn hello_html() -> &'static str {
    include_str!("widget/hello.html")
}
