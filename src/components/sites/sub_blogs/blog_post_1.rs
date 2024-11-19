use yew::prelude::*;

#[function_component(BlogPost1)]
pub fn blog_post1() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 1" }</h1>
            <p class="lead">{ "This is the content of blog post 1." }</p>
        </div>
    }
}


