use gray_matter::engine::YAML;
use gray_matter::Matter;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use log::error;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub image: String,
    pub date: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub metadata: PostMetadata,
    pub route: String,
    pub html: String,
}

impl Post {
    fn new(metadata: PostMetadata, route: String, html: String) -> Self {
        Post {
            metadata,
            route,
            html,
        }
    }
}

const POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/posts/");

lazy_static! {
    static ref POSTS: Vec<Post> = {
        let files = POSTS_DIR.files();
        let mut posts = Vec::new();
        let matter = Matter::<YAML>::new();
        for f in files {
            let result = matter.parse(std::str::from_utf8(f.contents()).expect("Invalid UTF-8"));
            let parser = Parser::new_ext(&result.content, Options::all());
            let mut html = String::new();
            html::push_html(&mut html, parser);
            let metadata: PostMetadata = match result.data {
                Some(data) => match data.deserialize() {
                    Ok(data) => data,
                    Err(_) => {
                        error!("Error parsing metadata for {}", f.path().to_str().unwrap());
                        panic!()
                    }
                },
                None => {
                    error!("Metadata not found for {}", f.path().to_str().unwrap());
                    panic!()
                }
            };
            posts.push(Post::new(
                metadata,
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

pub fn get_routes() -> Vec<&'static str> {
    let mut routes = Vec::with_capacity(POSTS.capacity());
    for post in POSTS.iter() {
        routes.push(post.route.as_str().clone());
    }
    routes
}

pub fn get_metadata() -> Vec<&'static PostMetadata> {
    let mut metadata = Vec::with_capacity(POSTS.capacity());
    for post in POSTS.iter() {
        metadata.push(&post.metadata);
    }
    metadata
}
