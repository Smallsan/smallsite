use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-5">
            <div class="row">
                <div class="col-md-6">
                    <div class="jumbotron fade-in">
                        <h1 class="display-4" style="font-weight: 700;">
                            <span style="background-color: #ff69b4; color: #fff;">{ "Yutori" }</span>{ ", remember to " }
                            <span style="color: #7CFC00;">{ "slow down" }</span>{ ". Coding isn't just about speed." }
                        </h1>
                        <p class="lead" style="font-weight: 700;">{ "It’s about clarity, efficiency, and building solutions that scale seamlessly." }</p>
                        <hr class="my-4" />
                        <Link<Route> to={Route::Projects} classes="btn btn-primary btn-lg custom-button slide-in-left">{ "See My Projects" }</Link<Route>>
                    </div>
                </div>
                <div class="col-md-6">
                    <div class="code-simulation mt-5 zoom-in">
                        <div class="code-header">
                            <span class="code-button close"></span>
                            <span class="code-button maximize"></span>
                            <span class="code-button minimize"></span>
                        </div>
                        <pre>
                            <code style="font-weight: 700;">
                                {"struct Coder {"}<span class="cursor">{"|"}</span>
                                {"\n    name: String,"}
                                {"\n    skills: Vec<&'static str>,"}
                                {"\n    full_stack: bool,"}
                                {"\n    fast_learner: bool,"}
                                {"\n}"}
                                {"\n"}
                                {"\n"}
                                {"fn get_coder_info() -> Coder {"}
                                {"\n    Coder {"}
                                {"\n        name: String::from(\"Master Coder\"),"}
                                {"\n        skills: vec![\"Rust\", \"Java\", \"Python\"],"}
                                {"\n        full_stack: true,"}
                                {"\n        fast_learner: true,"}
                                {"\n    };"}
                                {"\n}"}
                                {"\n"}
                            </code>
                        </pre>
                    </div>
                </div>
            </div>
            <style>
                {"
                    @import url('https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap');

                    .custom-button {
                        font-family: 'Roboto', sans-serif;
                    }

                    .fade-in {
                        animation: fadeIn 2s ease-in-out;
                    }

                    .slide-in-left {
                        animation: slideInLeft 2s ease-in-out;
                    }

                    .zoom-in {
                        animation: zoomIn 2s ease-in-out;
                    }

                    @keyframes fadeIn {
                        from {
                            opacity: 0;
                        }
                        to {
                            opacity: 1;
                        }
                    }

                    @keyframes slideInLeft {
                        from {
                            transform: translateX(-100%);
                            opacity: 0;
                        }
                        to {
                            transform: translateX(0);
                            opacity: 1;
                        }
                    }

                    @keyframes zoomIn {
                        from {
                            transform: scale(0);
                            opacity: 0;
                        }
                        to {
                            transform: scale(1);
                            opacity: 1;
                        }
                    }

                    .code-simulation {
                        background-color: #282c34;
                        color: #61dafb;
                        padding: 20px;
                        border-radius: 5px;
                        font-family: 'Courier New', Courier, monospace;
                        position: relative;
                        border: 2px solid #8A2BE2; /* Void purple tint */
                    }
                    .code-header {
                        display: flex;
                        justify-content: flex-end;
                        margin-bottom: 10px;
                    }
                    .code-button {
                        width: 12px;
                        height: 12px;
                        border-radius: 50%;
                        margin-left: 5px;
                    }
                    .code-button.close {
                        background-color: #ff5f56; /* Red */
                    }
                    .code-button.maximize {
                        background-color: #ffbd2e; /* Yellow */
                    }
                    .code-button.minimize {
                        background-color: #27c93f; /* Green */
                    }
                    .code-simulation .cursor {
                        display: inline-block;
                        width: 10px;
                        background-color: #61dafb;
                        animation: blink 1s step-end infinite;
                    }
                    @keyframes blink {
                        from, to {
                            background-color: transparent;
                        }
                        50% {
                            background-color: #61dafb;
                        }
                    }
                "}
            </style>
        </div>
    }
}