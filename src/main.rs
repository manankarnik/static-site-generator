use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <main>
            <h1>{"Hello World!"}</h1>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
