use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="p-4 text-center text-gray-400 border border-t border-gray font-head">
            {"\u{00a9} 2023 Copyright: Blog"}
        </footer>
    }
}
