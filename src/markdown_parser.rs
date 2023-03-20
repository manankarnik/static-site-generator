use include_dir::{include_dir, Dir};
use pulldown_cmark::{html, Options, Parser};
use std::cell::RefCell;
use yew::prelude::*;

thread_local! {static POSTS: RefCell<Vec<Post>> = RefCell::new(Vec::new());}
const POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/posts/");

#[derive(Debug, Clone)]
pub struct Post {
    pub route: AttrValue,
    pub html: AttrValue,
}

impl Post {
    fn new(route: AttrValue, html: AttrValue) -> Self {
        Post { route, html }
    }
}
pub fn parse() {
    let files = POSTS_DIR.files();
    for f in files {
        let parser = Parser::new_ext(
            std::str::from_utf8(f.contents()).expect("Invalid UTF-8"),
            Options::all(),
        );
        let mut html = String::new();
        html::push_html(&mut html, parser);
        POSTS.with(|p| {
            p.borrow_mut().push(Post::new(
                f.path()
                    .file_stem()
                    .expect("No file name")
                    .to_str()
                    .expect("Invalid unicode")
                    .to_string()
                    .into(),
                html.into(),
            ))
        });
    }
}

pub fn get_posts() -> Vec<Post> {
    POSTS.with(|p| p.borrow().clone().to_vec())
}

pub fn get_post(route: &str) -> Option<Post> {
    get_posts().iter().find(|p| p.route == route).cloned()
}
