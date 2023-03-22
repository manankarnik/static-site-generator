use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub content: AttrValue,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let raw_html = String::from("<article>\n") + props.content.clone().as_str() + "</article>";
    Html::from_html_unchecked(AttrValue::from(raw_html))
}
