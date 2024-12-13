use comrak::Options;
use maud::{html, Markup, PreEscaped};

const ABOUT_ME: &str = include_str!("../assets/aboutme.md");

pub fn homepage() -> Markup {
    let about_me = comrak::markdown_to_html(ABOUT_ME, &Options::default());
    html! {
        div class="content" { p {(PreEscaped(about_me)) } }
    }
}
