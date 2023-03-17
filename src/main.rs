use yew::prelude::*;
mod components;
use components::card::Card;
use components::header::Header;
use components::hero::Hero;

#[function_component(App)]
fn app() -> Html {
    let card_placeholder = (
        "17 Mar, 2023",
        "Lorem Ipsum",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce nec hendrerit elit.",
    );
    html! {
        <>
            <Header />
            <main class="mt-28 m-8 font-body">
                <Hero />
                <section class="my-8 grid grid-cols-3 gap-8">
                    { for (1..10).into_iter().map(|_| html! {<Card date={card_placeholder.0} title={card_placeholder.1} summary={card_placeholder.2}/>})}
                </section>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
