#![warn(clippy::all, clippy::pedantic)]
use std::error::Error;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum PageErrors {
    PageNotFound(String),
}

impl Display for PageErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageErrors::PageNotFound(e) => write!(f, "Page \"{e}\" not found."),
        }
    }
}

impl Error for PageErrors {
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Pages {
    Home,
    Blog(Option<String>),
    Photography,
    Github,
}
impl Pages {
    /// Returns the route belonging to this [`Pages`].
    pub fn get_route(&self) -> &str {
        match self {
            Pages::Home => "/",
            Pages::Blog(_) => "/blog",
            Pages::Photography => "/photography",
            Pages::Github => "/github",
        }
    }

    pub fn parse_route(url: &str) -> Result<Self, PageErrors> {
        println!("{url}");
        let segments = url.split('/').collect::<Box<[&str]>>();
        match *segments {
            ["", ""] => Ok(Pages::Home),
            ["", route] => Ok(Self::parse_1_route(route)?),
            ["", route, subroute] => Ok(Self::parse_2_route(route, subroute)?),

            _ => Err(PageErrors::PageNotFound("Route unavailable!".to_string())),
        }
    }

    fn parse_1_route(route: &str) -> Result<Self, PageErrors> {
        match route {
            "photography" => Ok(Self::Photography),
            "github" | "gh" => Ok(Self::Github),
            "blog" => Ok(Self::Blog(None)),
            e => Err(PageErrors::PageNotFound(e.to_string())),
        }
    }

    fn parse_2_route(route: &str, subroute: &str) -> Result<Self, PageErrors> {
        match route {
            "blog" => Ok(Self::Blog(Some(subroute.to_string()))),
            _ => Err(PageErrors::PageNotFound(format!("{route}/{subroute}"))),
        }
    }

}

impl Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pages::Home => write!(f, "Home"),
            Pages::Blog(p) => write!(f, "Blog {}", p.clone().unwrap_or(String::new())),
            Pages::Photography => write!(f, "Photography"),
            Pages::Github => write!(f, "Github"),
        }
    }
}

#[test]
fn test_pages_fmt() {
    assert_eq!(Pages::Home.to_string(), "Home");
    //assert_eq!(Pages::Blog.to_string(), "Blog");
    assert_eq!(Pages::Photography.to_string(), "Photography");
    assert_eq!(Pages::Github.to_string(), "Github");
}

//#[test]
//fn test_get_route() {
//    assert_eq!(Pages::Home.get_route(), "/");
//    //assert_eq!(Pages::Blog.get_route(), "/blog");
//    assert_eq!(Pages::Photography.get_route(), "/photography");
//    assert_eq!(Pages::Github.get_route(), "/github")
//}

