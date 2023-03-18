use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
mod router;
use components::footer::Footer;
use components::header::Header;
use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="mt-28 m-8 font-body">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </main>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
