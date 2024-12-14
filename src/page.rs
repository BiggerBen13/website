use std::error::Error;

use strum::IntoEnumIterator;

use maud::{html, Markup, DOCTYPE};
use tiny_http::Response;

use crate::{
    home, photography, util::{content, routes::Pages}
};

const CSS: &str = "style/style.css";

// This is the main function which handles requests
pub fn handle_request(request: tiny_http::Request) -> Result<(), Box<dyn Error>> {
    match Pages::get_page(request.url()) {
        // Request to one of the valid pages
        Ok(p) => {
            let page: String = p.build_page()?.into_string();
            let response = Response::from_string(page);
            let response = response.with_header(
                "Content-Type: text/html"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
            request.respond(response)?;
            return Ok(());
        }

        // Request to either content or nothing
        Err(_) => {
            match content::serve_function(&request) {
                Ok(r) => {
                    request.respond(r)?;
                }
                Err(_) => {
                    request.respond(tiny_http::Response::empty(tiny_http::StatusCode(405)))?;
                }
            };
        }
    };
    Ok(())
}

impl Pages {
    // Contructs a page from the given page enum
    fn build_page(&self) -> Result<Markup, Box<dyn Error>> {
        let embed_page = match self {
            Pages::Home => home::homepage,
            //Pages::Blog => blog::blog_page,
            Pages::Photography => photography::photography_page,
            Pages::Github => || {html!{ meta http-equiv="Refresh" content="0, URL=https://github.com/BiggerBen13";}},
            //_ => todo!(),
        };

        let markup = html! {
            (DOCTYPE)
            title {(format!("BiggerBen: {}", self))}
            link rel="stylesheet" type="text/css" href=(CSS);
            body {
                div class="term" {
                    (generate_navbar(self))
                    (embed_page())
                }
            }
        };
        Ok(markup)
    }
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

