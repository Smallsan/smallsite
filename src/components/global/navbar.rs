use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <>
            <style>
                {"
                    @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap');

                    body {
                        font-family: 'Roboto', sans-serif;
                    }

                    .navbar-toggler-icon {
                        width: 20px;
                        height: 20px;
                    }

                    .signup-btn {
                        border: 2px solid white;
                        border-radius: 20px;
                        padding: 5px 15px;
                        color: black;
                        background-color: white;
                    }

                    .navbar-collapse {
                        background-color: rgba(0, 0, 0, 0.5);
                        backdrop-filter: blur(10px); /* Apply blur effect */
                        position: absolute;
                        top: 100%;
                        left: 0;
                        right: 0;
                        z-index: 1;
                        padding-left: 30px; /* Add padding to move items to the right */
                    }

                    @media (min-width: 992px) {
                        .navbar-collapse {
                            position: static;
                            background-color: transparent;
                            backdrop-filter: none;
                            padding-left: 0;
                        }
                    }
                "}
            </style>
            <nav class="navbar navbar-expand-lg navbar-dark sticky-top">
                <div class="container">
                    <Link<Route> to={Route::Home} classes="navbar-brand d-flex align-items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" width="30" height="30" fill="currentColor" class="bi bi-bootstrap" viewBox="0 0 16 16">
                            <path d="M5.062 12.93c-.5 0-.937-.188-1.312-.563-.375-.375-.563-.812-.563-1.312V5.25c0-.5.188-.937.563-1.312.375-.375.812-.563 1.312-.563h3.625c.5 0 .937.188 1.312.563.375.375.563.812.563 1.312 0 .5-.188.937-.563 1.312-.375.375-.812.563-1.312.563H5.062v1.125h3.625c.5 0 .937.188 1.312.563.375.375.563.812.563 1.312 0 .5-.188.937-.563 1.312-.375.375-.812.563-1.312.563H5.062zm0-6.75h3.625c.25 0 .438-.062.563-.188.125-.125.188-.312.188-.563 0-.25-.062-.438-.188-.563-.125-.125-.312-.188-.563-.188H5.062c-.25 0-.438.062-.563.188-.125.125-.188.312-.188.563 0 .25.062.438.188.563.125.125.312.188.563.188zm0 4.5h3.625c.25 0 .438-.062.563-.188.125-.125.188-.312.188-.563 0-.25-.062-.438-.188-.563-.125-.125-.312-.188-.563-.188H5.062c-.25 0-.438.062-.563.188-.125.125-.188.312-.188.563 0 .25.062.438.188.563.125.125.312.188.563.188z"/>
                            <path d="M0 4a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v8a4 4 0 0 1-4 4H4a4 4 0 0 1-4-4V4z"/>
                        </svg>
                        <span class="d-none d-lg-inline ms-2">{ "Kurisu" }</span>
                    </Link<Route>>
                    <div class="d-flex order-lg-2 mx-auto">
                        <ul class="navbar-nav d-flex flex-row">
                            <li class="nav-item me-3"><a class="nav-link" href="#">{ "Login" }</a></li>
                            <li class="nav-item"><a class="nav-link signup-btn" href="#">{ "Sign up" }</a></li>
                        </ul>
                    </div>
                    <button class="navbar-toggler order-lg-3" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse justify-content-center order-lg-1" id="navbarNav">
                        <ul class="navbar-nav">
                            <li class="nav-item"><Link<Route> to={Route::Home} classes="nav-link">{ "Home" }</Link<Route>></li>
                            <li class="nav-item"><Link<Route> to={Route::About} classes="nav-link">{ "About" }</Link<Route>></li>
                            <li class="nav-item"><Link<Route> to={Route::Skills} classes="nav-link">{ "Skills" }</Link<Route>></li>
                            <li class="nav-item"><Link<Route> to={Route::Projects} classes="nav-link">{ "Projects" }</Link<Route>></li>
                            <li class="nav-item"><Link<Route> to={Route::Blog} classes="nav-link">{ "Blog" }</Link<Route>></li>
                            <li class="nav-item"><Link<Route> to={Route::Contact} classes="nav-link">{ "Contact" }</Link<Route>></li>
                        </ul>
                    </div>
                </div>
            </nav>
        </>
    }
}