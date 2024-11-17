use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-5">
            <div class="jumbotron">
                <h1 class="display-4">{ "Welcome to My Portfolio" }</h1>
                <p class="lead">{ "I'm a passionate developer with experience in building web applications. Explore my projects and get to know more about me." }</p>
                <hr class="my-4" />
                <p>{ "This portfolio showcases some of my best work and achievements. Feel free to reach out if you have any questions or would like to collaborate." }</p>
                <a class="btn btn-primary btn-lg" href="#/projects" role="button">{ "View Projects" }</a>
                <a class="btn btn-secondary btn-lg ml-2" href="#/contact" role="button">{ "Contact Me" }</a>
            </div>
        </div>
    }
}