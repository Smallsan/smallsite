use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "About" }</h1>
            <p class="lead">{ "This is the about page." }</p>
        </div>
    }
}