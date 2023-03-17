use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="my-8 flex justify-center items-center h-60 bg-slate-200 rounded-lg">
            <h1 class="text-4xl font-bold">{"Welcome to the Blog!"}</h1>
        </section>
    }
}
