use std::{error::Error, fmt::Display, fs, path::Path};

use ascii::AsciiString;

#[derive(Debug)]
enum ContentError {
    FileNotFound(String),
}

impl Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentError::FileNotFound(s) => write!(f, "File {s} not found!"),
        }
    }
}

impl Error for ContentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match &extension.to_str().unwrap().to_lowercase()[0..] {
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" | "html" => "text/html; charset=utf8",
        "css" => "text/css; charset=utf8",
        _ => "text/plain; charset=utf8",
    }
}

pub fn serve_function(
    request: &tiny_http::Request,
) -> Result<tiny_http::Response<fs::File>, Box<dyn Error>> {
    println!("Handling request: {request:?}");
    let url = &request.url()[1..];
    let path = Path::new(&url);
    // println!("{}", url);
    let file = fs::File::open(path);

    match file {
        Ok(f) => {
            let response = tiny_http::Response::from_file(f);
            let response = response.with_header(tiny_http::Header {
                field: "Content-Type".parse().unwrap(),
                value: AsciiString::from_ascii(get_content_type(path)).unwrap(),
            });
            Ok(response)
        }
        Err(_) => Err(Box::new(ContentError::FileNotFound(url.into()))),
    }
}
