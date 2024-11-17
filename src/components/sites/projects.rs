use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Projects" }</h1>
            <p class="lead">{ "This is the projects page." }</p>
        </div>
    }
}
