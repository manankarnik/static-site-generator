use crate::markdown_parser;
use crate::pages::about::About;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::post::Post;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/post/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Blog => html! { <p>{"Blog"}</p> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Post { id } => match markdown_parser::get_post(&id) {
            Some(post) => html! { <Post content={post.html} /> },
            None => html! { <p>{"Error 404: Post not found"}</p> },
        },
        Route::NotFound => html! { <p>{"Error 404"}</p> },
    }
}
