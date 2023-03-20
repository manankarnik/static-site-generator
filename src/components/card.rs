use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub id: AttrValue,
    pub date: AttrValue,
    pub title: AttrValue,
    pub summary: AttrValue,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <a href={"post/".to_owned() + props.id.as_str()} class="shadow-md group">
            <div class="overflow-hidden">
                <img src="https://dummyimage.com/640x360/eee/aaa" class="group-hover:scale-110 transition-all duration-500" />
            </div>
            <div class="p-4">
                <span class="text-gray-400">{&props.date}</span>
                <h2>{&props.title}</h2>
                <p>{&props.summary}</p>
            </div>
        </a>
    }
}
