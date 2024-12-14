#[warn(clippy::all, clippy::pedantic)]
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
            PageErrors::PageNotFound(e) => write!(f, "Page \"{}\" not found.", e),
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
    //Blog,
    Photography,
    Github,
}
impl Pages {
    /// Returns the route belonging to this [`Pages`].
    pub fn get_route(&self) -> &str {
        match self {
            Pages::Home => "/",
            //Pages::Blog => "/blog",
            Pages::Photography => "/photography",
            Pages::Github => "/github",
        }
    }

    /// This function returns the page belonging to a route.
    ///
    /// # Errors
    ///
    /// This function will return an error if the page doesn't exist.
    pub fn get_page(route: &str) -> Result<Self, PageErrors> {
        match route {
            "/" => Ok(Pages::Home),
            //"/blog" => Ok(Pages::Blog),
            "/photography" => Ok(Pages::Photography),
            "/github" => Ok(Pages::Github),
            _ => Err(PageErrors::PageNotFound(route.into())),
        }
    }
}

impl Display for Pages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pages::Home => write!(f, "Home"),
            //Pages::Blog => write!(f, "Blog"),
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

#[test]
fn test_get_route() {
    assert_eq!(Pages::Home.get_route(), "/");
    //assert_eq!(Pages::Blog.get_route(), "/blog");
    assert_eq!(Pages::Photography.get_route(), "/photography");
    assert_eq!(Pages::Github.get_route(), "/github")
}

#[test]
fn test_get_page() {
    assert_eq!(Pages::get_page("/").unwrap(), Pages::Home);
    //assert_eq!(Pages::get_page("/blog").unwrap(), Pages::Blog);
    assert_eq!(Pages::get_page("/photography").unwrap(), Pages::Photography);
    assert_eq!(Pages::get_page("/github").unwrap(), Pages::Github);
}
