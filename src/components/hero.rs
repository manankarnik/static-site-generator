use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="my-8 flex justify-center items-center h-60 bg-zinc-200 dark:bg-zinc-800 rounded-lg">
            <h1>{"Welcome to the Blog!"}</h1>
        </section>
    }
}
