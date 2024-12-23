#![warn(clippy::all, clippy::pedantic, clippy::perf)]
use comrak::{arena_tree::Node, nodes, ComrakOptions};

use std::{
    collections::HashMap,
    fs::read_dir,
    io::BufWriter,
    sync::{LazyLock, Mutex},
};

use maud::PreEscaped;

pub static BLOGS: LazyLock<Mutex<HashMap<String, Blog>>> = LazyLock::new(|| generage_blogs());

pub struct Blog {
    pub(crate) title: String,
    pub(crate) date: chrono::NaiveDate,
    pub(crate) content: PreEscaped<String>,
}

#[derive(Debug)]
struct MetaData {
    date: chrono::NaiveDate,
    title: String,
}

impl Default for MetaData {
    fn default() -> Self {
        let date = chrono::NaiveDate::default();
        let title = "untitled".to_owned();
        Self { date, title }
    }
}

#[derive(Debug)]
enum FrontMatter {
    Date(chrono::NaiveDate),
    Title(String),
}

impl Blog {
    fn new(title: String, date: chrono::NaiveDate, content: PreEscaped<String>) -> Self {
        Self {
            title,
            date,
            content,
        }
    }
}

fn generage_blogs() -> Mutex<HashMap<String, Blog>> {
    let mut blog_map = HashMap::<String, Blog>::new();
    let mut options = ComrakOptions::default();
    options.extension.front_matter_delimiter = Some("---".to_owned());
    let arena = comrak::Arena::new();
    let blogs = read_dir("blogs").expect("Blog directory not found please create!");
    for blog in blogs {
        let Ok(file_contents) = std::fs::read_to_string(blog.expect("Invalid path!").path()) else {
            continue;
        };
        let root = comrak::parse_document(&arena, &file_contents, &options);
        let metadata = parse_frontmatter(&get_frontmatter(root).unwrap());
        let blog = {
            let mut buf = BufWriter::new(Vec::new());
            comrak::format_html(root, &options, &mut buf);
            let content = String::from_utf8(buf.into_inner().unwrap()).unwrap();
            Blog {
                title: metadata.title,
                date: metadata.date,
                content: PreEscaped(content),
            }
        };
        blog_map.insert(blog.title.clone(), blog);
    }
    Mutex::new(blog_map)
}

fn parse_frontmatter(front_matter: &str) -> MetaData {
    let front_matter = front_matter.trim_matches('-');
    let front_matter = front_matter.lines();
    let mut metadata = MetaData::default();
    'parse_loop: for line in front_matter {
        let tokens = line.split(":").map(|x| x.trim()).collect::<Box<[&str]>>();
        match *tokens {
            ["date", date] => {
                let Ok(date) = chrono::NaiveDate::parse_from_str(date, "%d-%m-%Y") else {
                    println!("unable to parse date");
                    continue 'parse_loop;
                };
                metadata.date = date
            }
            ["title", title] => metadata.title = title.to_owned(),
            _ => (),
        }
    }

    metadata
}

#[inline(always)]
fn get_frontmatter<'a>(root: &'a Node<'a, std::cell::RefCell<nodes::Ast>>) -> Option<String> {
    for node in root.children() {
        match node.data.clone().into_inner().value {
            nodes::NodeValue::FrontMatter(front_matter) => {
                return Some(front_matter);
            }
            _ => continue,
        }
    }

    None
}
