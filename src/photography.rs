use std::sync::Mutex;

use maud::{html, Markup};

static PHOTO_PATHS: std::sync::LazyLock<Mutex<Vec<String>>> = std::sync::LazyLock::new(|| {
    let Ok(photos) = std::fs::read_dir("photos") else {
        return Mutex::new(Vec::<String>::new());
    };

    let mut photo_paths: Vec<String> = Vec::<String>::new();

    for p in photos {
        match p {
            Ok(p) => match p.path().into_os_string().into_string() {
                Ok(s) => {
                    photo_paths.push(s);
                }
                Err(_) => return Mutex::new(Vec::<String>::new()),
            },
            Err(_) => return Mutex::new(Vec::<String>::new()),
        };
    }

    Mutex::new(photo_paths)
});

pub fn page() -> Markup {
    html! {
            div class="content" {
                h1 { "Photos" }
                div class="photography_board" {
                    (generate_photos())
            }
        }
    }
}

pub fn generate_photos() -> Markup {
    html! {
        @for p in PHOTO_PATHS.lock().unwrap().iter() {
            div class="image" {
                a href=(p) { img src=(p) alt=""; }
            }
        };
    }
}
