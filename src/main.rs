use yew::prelude::*;
mod components;
use components::header::Header;
use components::hero::Hero;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="mt-28 m-8">
                <Hero />
            </main>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
