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
        <header class="px-8 flex items-center fixed top-0 w-full h-20 border-b border-grey backdrop-blur-lg font-head uppercase">
            <a href="/" class="w-full text-4xl font-bold">{"Blog"}</a>
            <ul class="w-full flex justify-center items-center">
                { for nav_links.iter().map(|item| { html! {<li class="p-4 text-lg"><a href={item.link}>{item.name}</a></li>} })}
            </ul>
            <ul class="w-full flex justify-end items-center">
                <li><a class="mx-2 py-2 px-4 text-lg rounded-xl">{"Log in"}</a></li>
                <li><a class="mx-2 py-2 px-4 bg-black text-white text-lg rounded-xl">{"Sign up"}</a></li>
            </ul>
        </header>
    }
}
