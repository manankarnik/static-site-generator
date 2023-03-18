use crate::pages::home::Home;
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
        Route::About => html! { <p>{"About"}</p> },
        Route::Contact => html! { <p>{"Contact"}</p> },
        Route::Post { id } => html! { <p>{format!("Post {id}")}</p> },
        Route::NotFound => html! { <p>{"Error 404"}</p> },
    }
}
