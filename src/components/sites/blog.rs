use yew::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog" }</h1>
            <p class="lead">{ "This is the blog page." }</p>
        </div>
    }
}
