use yew::prelude::*;

#[function_component(BlogPost2)]
pub fn blog_post2() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 2" }</h1>
            <p class="lead">{ "This is the content of blog post 2." }</p>
        </div>
    }
}
