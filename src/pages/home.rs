use crate::components::card::Card;
use crate::components::hero::Hero;
use crate::markdown_parser;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let routes = markdown_parser::get_routes();
    let metadata = markdown_parser::get_metadata();
    html! {
        <>
            <Hero />
            <section class="my-8 grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-8">
                { for (0..routes.len()).into_iter().map(|id| html! {<Card id={routes[id]} date={metadata[id].date.clone()} title={metadata[id].title.clone()} summary={metadata[id].summary.clone()} img_src={metadata[id].image.clone()} />})}
            </section>
        </>
    }
}
