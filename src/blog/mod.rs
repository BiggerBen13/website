use maud::{html, Markup};

pub fn blog_page(blog: Option<String>) -> Markup {
    match blog {
        None => html!("Hello"),
        Some(b) => html!("Hello" (b))
    }
}
