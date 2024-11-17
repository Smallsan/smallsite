use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Contact" }</h1>
            <p class="lead">{ "This is the contact page." }</p>
        </div>
    }
}