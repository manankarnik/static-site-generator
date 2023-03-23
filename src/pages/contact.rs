use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section class="md:p-8 md:mx-auto md:w-3/4 md:shadow-md md:bg-zinc-50 md:dark:bg-zinc-800">
            <h1>{"Contact"}</h1>
            <form class="flex flex-col justify-center gap-4 mb-8 [&>:not(input[type='submit'])]:p-2 [&>:not(input[type='submit'])]:border [&>:not(input[type='submit'])]:border-gray-200 [&>:not(input[type='submit'])]:dark:border-gray-700 [&>:not(input[type='submit'])]:dark:bg-zinc-700">
                <input type="text" placeholder="Name"/>
                <input type="email" placeholder="Email"/>
                <textarea rows="4" placeholder="Message" />
                <input type="submit" class="p-3 bg-red-500 hover:bg-red-600 text-white" />
            </form>
        </section>
    }
}
