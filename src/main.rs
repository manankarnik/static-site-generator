use yew::prelude::*;
mod components;
use components::header::Header;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="mt-20 p-4">
                <h1 class="text-3xl font-bold underline">{"Hello World!"}</h1>
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
