use yew::prelude::*;

#[function_component(OctQuant)]
pub fn octquant() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Octquant" }</h1>
            <p class="lead">{ "This is the Octquant page." }</p>
        </div>
    }
}