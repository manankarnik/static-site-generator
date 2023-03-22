use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser};

#[derive(Debug, Clone)]
pub struct Post {
    pub route: String,
    pub html: String,
}

impl Post {
    fn new(route: String, html: String) -> Self {
        Post { route, html }
    }
}

const POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/posts/");

lazy_static! {
    static ref POSTS: Vec<Post> = {
        let files = POSTS_DIR.files();
        let mut posts = Vec::new();
        for f in files {
            let parser = Parser::new_ext(
                std::str::from_utf8(f.contents()).expect("Invalid UTF-8"),
                Options::all(),
            );
            let mut html = String::new();
            html::push_html(&mut html, parser);
            posts.push(Post::new(
                f.path()
                    .file_stem()
                    .expect("No file name")
                    .to_str()
                    .expect("Invalid unicode")
                    .to_string(),
                html,
            ));
        }
        posts
    };
}

pub fn get_post(route: &str) -> Option<Post> {
    POSTS.iter().find(|p| p.route == route).cloned()
}
