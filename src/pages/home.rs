use crate::components::card::Card;
use crate::components::hero::Hero;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let card_placeholder = (
        "17 Mar, 2023",
        "Lorem Ipsum",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce nec hendrerit elit.",
    );
    html! {
        <>
            <Hero />
            <section class="my-8 grid grid-cols-3 gap-8">
                { for (1..10).into_iter().map(|_| html! {<Card date={card_placeholder.0} title={card_placeholder.1} summary={card_placeholder.2}/>})}
            </section>
        </>
    }
}
