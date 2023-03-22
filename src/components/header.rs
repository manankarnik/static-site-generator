use super::theme_switcher::ThemeSwitcher;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

struct NavItem {
    name: &'static str,
    link: &'static str,
}

#[function_component(Header)]
pub fn header() -> Html {
    let nav_links = vec![
        NavItem {
            name: "Home",
            link: "/",
        },
        NavItem {
            name: "Blog",
            link: "/blog",
        },
        NavItem {
            name: "About",
            link: "/about",
        },
        NavItem {
            name: "Contact",
            link: "/contact",
        },
    ];

    let overlay_classes = use_state(|| {
        vec![
            "md:hidden",
            "absolute",
            "top-20",
            "left-0",
            "w-screen",
            "h-screen",
            "bg-white/50",
            "dark:bg-black/50",
            "hidden",
        ]
    });

    let menu_classes = use_state(|| {
        vec![
            "md:hidden",
            "absolute",
            "top-20",
            "right-0",
            "w-[50vw]",
            "h-screen",
            "flex",
            "flex-col",
            "bg-white",
            "dark:bg-zinc-900",
            "transition-all",
            "translate-x-full",
        ]
    });

    let body_element = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document")
        .query_selector("body")
        .expect("No body")
        .unwrap();

    let on_menu_click = {
        let overlay_classes = overlay_classes.clone();
        let menu_classes = menu_classes.clone();
        let body_element = body_element.clone();
        Callback::from(move |_| {
            let mut overlay_classes_vec = (*overlay_classes).clone();
            let mut menu_classes_vec = (*menu_classes).clone();
            if (*overlay_classes).contains(&"hidden") {
                overlay_classes_vec.retain(|css_class| css_class.to_owned() != "hidden");
                menu_classes_vec.retain(|css_class| css_class.to_owned() != "translate-x-full");
                body_element.set_class_name("overflow-hidden");
            } else {
                overlay_classes_vec.push("hidden");
                menu_classes_vec.push("translate-x-full");
                body_element.set_class_name("");
            }
            overlay_classes.set(overlay_classes_vec);
            menu_classes.set(menu_classes_vec);
        })
    };

    let on_overlay_click = {
        let overlay_classes = overlay_classes.clone();
        let menu_classes = menu_classes.clone();
        let body_element = body_element.clone();
        Callback::from(move |_| {
            let mut overlay_classes_vec = (*overlay_classes).clone();
            let mut menu_classes_vec = (*menu_classes).clone();
            overlay_classes_vec.push("hidden");
            menu_classes_vec.push("translate-x-full");
            body_element.set_class_name("");
            overlay_classes.set(overlay_classes_vec);
            menu_classes.set(menu_classes_vec);
        })
    };

    html! {
        <header class="z-10 px-8 fixed top-0 w-full h-20 bg-white dark:bg-zinc-900 drop-shadow-lg font-head uppercase">
            <div class="flex h-full items-center">
                <div class="w-full flex">
                    <a href="/" class="text-3xl font-bold">{"Blog"}</a>
                </div>
                <nav class="hidden md:flex justify-center items-center">
                    { for nav_links.iter().map(|item| { html! {<a href={item.link} class="p-4 text-lg">{item.name}</a>} })}
                </nav>
                <ul class="w-full flex justify-end items-center">
                    <li><ThemeSwitcher /></li>
                    <li class="md:hidden"><button class="p-2" onclick={on_menu_click}><Icon icon_id={IconId::FontAwesomeSolidBars} /></button></li>
                </ul>
            </div>
            <div onclick={on_overlay_click} class={classes!((*overlay_classes).clone())}>
            </div>
            <nav class={classes!((*menu_classes).clone())}>
                { for nav_links.iter().map(|item| { html! {<a href={item.link} class="p-4 text-lg">{item.name}</a>} })}
            </nav>
        </header>
    }
}
