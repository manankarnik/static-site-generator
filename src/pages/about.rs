use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="lg:flex lg:items-center lg:justify-center lg:gap-8">
            <img src="https://dummyimage.com/400x400/eee/aaa" alt="author" class="object-contain mx-auto lg:w-[40%] h-full" />
            <article>
                <h1>{"About Me"}</h1>
                <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque molestie convallis odio, a condimentum ex porttitor id. Mauris gravida id nisi et molestie. In mattis venenatis mauris, at placerat tortor suscipit at. Mauris non nibh rhoncus, accumsan enim at, interdum leo. Vestibulum mattis dui lobortis congue maximus. Cras quis rutrum orci. Ut dignissim finibus lectus id sodales. Aenean a neque luctus, malesuada magna ac, tristique leo. Duis eget auctor augue. Nulla a quam leo. Aenean nec diam et ipsum faucibus condimentum. Nullam dui lorem, cursus eu ex quis, blandit pharetra risus. Mauris et sodales enim. Nunc et nisl massa. Etiam nec fringilla nunc. Integer vehicula quis velit quis rhoncus."}</p>

                <p>{"Sed orci ante, eleifend sed viverra id, mollis in neque. Sed id sapien eu metus varius volutpat non nec metus. Cras nec augue quis lorem mattis volutpat nec sed libero. Morbi iaculis, diam non venenatis ultricies, quam quam aliquet urna, ac pretium neque nunc ac elit. Cras finibus lectus tempus mi volutpat, eu sagittis nulla vulputate. Vestibulum eu ligula ut ante sagittis ultricies. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Vestibulum volutpat lorem id nisl convallis eleifend sit amet sodales augue. Nunc arcu nisl, varius at rhoncus non, luctus at ipsum. Vivamus orci lectus, cursus consectetur porttitor nec, vehicula et dui. Quisque nec magna semper, dignissim neque vitae, feugiat eros. Nam id metus tempor, rhoncus tortor quis, egestas nulla. Mauris et tincidunt nibh. Vestibulum cursus nisl ac cursus dapibus."}</p>
            </article>
        </section>
    }
}
