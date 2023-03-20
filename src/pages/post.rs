use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub content: AttrValue,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    Html::from_html_unchecked(props.content.clone())
}
