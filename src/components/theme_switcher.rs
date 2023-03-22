use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let is_dark =
        use_state(|| LocalStorage::get::<String>("theme").ok().as_deref() != Some("light"));
    let onclick = {
        let is_dark = is_dark.clone();
        let html_element = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document")
            .query_selector("html")
            .expect("No html")
            .unwrap();
        if (*is_dark).clone() {
            html_element.set_class_name("dark");
        } else {
            html_element.set_class_name("");
        }
        Callback::from(move |_| {
            is_dark.set(!(*is_dark).clone());
            LocalStorage::set("theme", if *is_dark { "light" } else { "dark" })
                .expect("Couldn't store theme");
        })
    };
    html! {
        <button {onclick} class="p-2">
            <Icon icon_id={if *is_dark {IconId::BootstrapSunFill} else {IconId::BootstrapMoonFill}} />
        </button>
    }
}
