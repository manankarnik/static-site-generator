use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="p-4 text-center text-gray-600 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700 font-head">
            {"\u{00a9} 2023 Copyright: Blog"}
        </footer>
    }
}
