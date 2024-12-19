use std::error::Error;

use strum::IntoEnumIterator;

use maud::{html, Markup, DOCTYPE};
use tiny_http::Response;

use crate::{
    blog, home, photography,
    util::{content, routes::Pages},
};

const CSS: &str = "/style/style.css";

// This is the main function which handles requests
pub fn handle_request(request: tiny_http::Request) -> Result<(), Box<dyn Error>> {
    let page = Pages::parse_route(request.url());

    match page {
        Ok(p) => {
            let page: String = build_page(p)?.into_string();
            let response = Response::from_string(page);
            let response = response.with_header(
                "Content-Type: text/html"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
            request.respond(response)?;
            return Ok(());
        }

        Err(_) => {
            let response = content::serve_function(&request)?;
            request.respond(response)?;
        }
    };
    Ok(())
}

// Contructs a page from the given page enum
fn build_page(page: Pages) -> Result<Markup, Box<dyn Error>> {
    let embed_page = match &page {
        Pages::Home => home::homepage(),
        Pages::Blog(blog) => blog::blog_page(blog.clone()),
        Pages::Photography => photography::photography_page(),
        Pages::Github => {
            html! { meta http-equiv="Refresh" content="0, URL=https://github.com/BiggerBen13"; }
        },
    };

    let markup = html! {
        (DOCTYPE)
        title {(format!("BiggerBen: {}", page))}
        link rel="stylesheet" type="text/css" href=(CSS);
        body {
            div class="term" {
                (generate_navbar(&page))
                (embed_page)
            }
        }
    };
    Ok(markup)
}

// Generates the NavBar
fn generate_navbar(current_page: &Pages) -> Markup {
    html! {
        div class="navbar" {
            @for page in Pages::iter() {
                    a href=(page.get_route()) class=(if page == *current_page { "selected" } else {""}){(page.to_string())}
            }
        }
    }
}
