use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container mt-5">
            <div class="row">
                <div class="col-md-6">
                    <div class="jumbotron">
                        <h1 class="display-4">{ "Welcome to My Portfolio" }</h1>
                        <p class="lead">{ "I'm a passionate developer with experience in building web applications." }</p>
                        <hr class="my-4" />
                        <p>{ "This portfolio showcases some of my best work and achievements. Feel free to reach out if you have any questions or would like to collaborate." }</p>
                    </div>
                </div>
                <div class="col-md-6">
                    <div class="code-simulation mt-5">
                        <pre>
                        <code>
                        {"struct Coder { "}<span class="cursor">{"|"}</span>
                        {"\n    name: String,"}
                        {"\n    skills: Vec<&'static str>,"}
                        {"\n    hard_worker: bool,"}
                        {"\n    creative: bool,"}
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
                    .code-simulation {
                        background-color: #282c34;
                        color: #61dafb;
                        padding: 20px;
                        border-radius: 5px;
                        font-family: 'Courier New', Courier, monospace;
                        position: relative;
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