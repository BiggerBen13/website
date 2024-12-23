use maud::{html, Markup};

mod generate;

pub fn page(blog: Option<String>) -> Markup {
    match blog {
        None => html! {(list_blogs())},
        Some(b) => {
            if let Some(b) = generate::BLOGS.lock().unwrap().get(&b) {
                html! { (b.content) }
            } else {
                html! { h1 { "NOT FOUND!" } }
            }
        }
    }
}

pub fn list_blogs() -> Markup {
    let binding = generate::BLOGS.lock().unwrap();

    let mut blogs = binding.iter().collect::<Vec<(_, &generate::Blog)>>();
    blogs.sort_by_key(|a| a.1.date);
    blogs.reverse();
    html! {
        @for blog in blogs.iter() {
            div { (blog.1.title) (blog.1.date)}

        }
    }
}
