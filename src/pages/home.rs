use crate::components::card::Card;
use crate::components::hero::Hero;
use crate::markdown_parser;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let routes = markdown_parser::get_routes();
    let card_placeholder = (
        "17 Mar, 2023",
        "Lorem Ipsum",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce nec hendrerit elit.",
    );
    html! {
        <>
            <Hero />
            <section class="my-8 grid grid-cols-3 gap-8">
                { for (0..routes.len()).into_iter().map(|id| html! {<Card id={routes[id]} date={card_placeholder.0} title={card_placeholder.1} summary={card_placeholder.2}/>})}
            </section>
        </>
    }
}
