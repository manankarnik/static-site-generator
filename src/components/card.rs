use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub date: AttrValue,
    pub title: AttrValue,
    pub summary: AttrValue,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="shadow-md">
            <img src="https://dummyimage.com/640x360/eee/aaa"/>
            <div class="p-4">
                <span class="text-gray-400">{&props.date}</span>
                <h2 class="text-4xl font-bold text-red-500">{&props.title}</h2>
                <p>{&props.summary}</p>
            </div>
        </div>
    }
}
