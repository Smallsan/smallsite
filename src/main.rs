use yew::prelude::*;
use yew_router::prelude::*;
mod components;
use components::sites::home::Home;
use components::sites::about::About;
use components::sites::projects::Projects;
use components::sites::blog::Blog;
use components::sites::contact::Contact;
use components::sites::skills::Skills;
use components::global::navbar::NavBar;



#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/skills")]
    Skills,
    #[at("/projects")]
    Projects,
    #[at("/blog")]
    Blog,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}



fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Skills => html! { <Skills /> },
        Route::Projects => html! { <Projects /> },
        Route::Blog => html! { <Blog /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}



#[function_component(Main)]
fn main() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}