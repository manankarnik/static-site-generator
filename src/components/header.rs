use yew::prelude::*;

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
    html! {
        <header class="z-10 px-8 flex items-center fixed top-0 w-full h-20 border-b border-grey backdrop-blur-lg font-head uppercase">
            <div class="w-full flex">
                <a href="/" class="text-4xl font-bold">{"Blog"}</a>
            </div>
            <nav class="hidden lg:flex justify-center items-center">
                { for nav_links.iter().map(|item| { html! {<a href={item.link} class="p-4 text-lg">{item.name}</a>} })}
            </nav>
            <nav>
            </nav>
            <ul class="w-full flex justify-end items-center">
            </ul>
        </header>
    }
}
