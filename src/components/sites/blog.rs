use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct BlogCardProps {
    pub title: String,
    pub description: String,
    pub link: Route,
}

#[function_component(BlogCard)]
fn blog_card(BlogCardProps { title, description, link }: &BlogCardProps) -> Html {
    html! {
        <div class="col-md-4 mb-4">
            <Link<Route> to={link.clone()} classes="text-decoration-none">
                <div class="card h-100">
                    <div class="card-body">
                        <h5 class="card-title">{ title }</h5>
                        <p class="card-text">{ description }</p>
                    </div>
                </div>
            </Link<Route>>
        </div>
    }
}

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog" }</h1>
            <div class="row">
                <BlogCard title="Blog Post 1" description="This is the first blog post." link={Route::BlogPost1} />
                <BlogCard title="Blog Post 2" description="This is the second blog post." link={Route::BlogPost2} />
                <BlogCard title="Blog Post 3" description="This is the third blog post." link={Route::BlogPost3} />
            </div>
        </div>
    }
}

#[function_component(BlogPost1)]
pub fn blog_post1() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 1" }</h1>
            <p class="lead">{ "This is the content of blog post 1." }</p>
        </div>
    }
}

#[function_component(BlogPost2)]
pub fn blog_post2() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 2" }</h1>
            <p class="lead">{ "This is the content of blog post 2." }</p>
        </div>
    }
}

#[function_component(BlogPost3)]
pub fn blog_post3() -> Html {
    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Blog Post 3" }</h1>
            <p class="lead">{ "This is the content of blog post 3." }</p>
        </div>
    }
}