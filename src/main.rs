use components::sites::sub_blogs::blog_post_1::BlogPost1;
use components::sites::sub_blogs::blog_post_2::BlogPost2;
use components::sites::sub_blogs::blog_post_3::BlogPost3;
use components::sites::sub_projects::fenpix::Fenpix;
use components::sites::sub_projects::octquant::OctQuant;
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
    #[at("/projects/fenpix")]
    Fenpix,
    #[at("/projects/octquant")]
    Octquant,
    #[at("/blog")]
    Blog,
    #[at("/blog/post1")]
    BlogPost1,
    #[at("/blog/post2")]
    BlogPost2,
    #[at("/blog/post3")]
    BlogPost3,
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
        Route::Fenpix => html! { <Fenpix /> },
        Route::Octquant => html! { <OctQuant /> },
        Route::BlogPost1 => html! { <BlogPost1 /> },
        Route::BlogPost2 => html! { <BlogPost2 /> },
        Route::BlogPost3 => html! { <BlogPost3 /> },
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