use yew::prelude::*;

#[function_component(BlogPost3)]
pub fn blog_post3() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 3" }</h1>
            <p class="lead">{ "This is the content of blog post 3." }</p>
        </div>
    }
}