use yew::prelude::*;

#[function_component(Fenpix)]
pub fn fenpix() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Fenpix" }</h1>
            <p class="lead">{ "This is the Fenpix page." }</p>
        </div>
    }
}