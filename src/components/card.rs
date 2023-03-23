use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub id: AttrValue,
    pub img_src: AttrValue,
    pub date: AttrValue,
    pub title: AttrValue,
    pub summary: AttrValue,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="shadow-md dark:bg-zinc-800 group">
            <Link<Route> to={Route::Post{id: props.id.to_string()}}>
                <div class="overflow-hidden">
                    <img src={&props.img_src} class="object-cover w-full h-full group-hover:scale-110 transition-all duration-500" />
                </div>
                <div class="p-4">
                    <span class="text-gray-600 dark:text-gray-400">{&props.date}</span>
                    <h2>{&props.title}</h2>
                    <p>{&props.summary}</p>
                </div>
            </Link<Route>>
        </div>
    }
}
