use log::info;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod markdown_parser;
mod pages;
mod router;
use components::footer::Footer;
use components::header::Header;
use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <main class="mt-28 m-8 font-body">
                    <Switch<Route> render={switch} />
            </main>
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("{:?}", markdown_parser::get_post("first"));
    yew::Renderer::<App>::new().render();
}
