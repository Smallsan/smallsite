use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
        <style>
        {"
        .card {
            position: relative;
            overflow: hidden;
            border: none;
            border-radius: 10px;
            transition: transform 0.3s, box-shadow 0.3s;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        }

        .card:hover {
            transform: scale(1.05);
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.2);
        }

        .card-img-top {
            width: 100%;
            height: auto;
            border-bottom: 1px solid #ddd;
            transition: opacity 0.3s;
        }

        .card-hover-img {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            opacity: 0;
            transition: opacity 0.3s;
        }

        .card:hover .card-hover-img {
            opacity: 1;
        }

        .card-body {
            position: absolute;
            bottom: 0;
            width: 100%;
            background: rgba(0, 0, 0, 0.7); /* Semi-transparent background */
            color: white;
            text-align: center;
            padding: 20px;
            box-sizing: border-box;
        }

        .card-title {
            margin-bottom: 10px;
            font-size: 1.25rem;
            font-weight: bold;
        }

        .card-text {
            margin-bottom: 0;
            font-size: 1rem;
        }

        .projects-container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }

        .row {
            display: flex;
            flex-wrap: wrap;
            margin: -10px;
        }

        .col-md-4 {
            flex: 0 0 33.3333%;
            max-width: 33.3333%;
            padding: 10px;
        }

        @media (max-width: 768px) {
            .col-md-4 {
                flex: 0 0 100%;
                max-width: 100%;
            }
        }
        "}
        </style>
        <div class="projects-container mt-5">
            <h1 class="display-4">{ "Projects" }</h1>
            <p class="lead">{ "This is the projects page." }</p>
            <div class="row">
                <div class="col-md-4 mb-4">
                    <Link<Route> to={Route::Fenpix} classes="text-decoration-none">
                        <div class="card h-100">
                            <img src="/static/assets/projects/fenpix.png" class="card-img-top" alt="Fenpix" />
                            <img src="/static/assets/projects/fenpix_hover.png" class="card-hover-img" alt="Fenpix Hover" />
                            <div class="card-body">
                                <h5 class="card-title">{ "Fenpix" }</h5>
                                <p class="card-text">{ "A Rust library that converts FEN strings into pixel chess boards." }</p>
                            </div>
                        </div>
                    </Link<Route>>
                </div>
                <div class="col-md-4 mb-4">
                    <a href="https://github.com/Smallsan/OctQuant"> 
                    <div class="card h-100">
                        <img src="/static/assets/projects/octree.png" class="card-img-top" alt="Octree" />
                        <img src="/static/assets/projects/octree_hover.png" class="card-hover-img" alt="Octree Hover" />
                        <div class="card-body">
                            <h5 class="card-title">{ "Octree Quant" }</h5>
                            <p class="card-text">{ "Octree color quantization algorithm in Go." }</p>
                        </div>
                    </div>
                    </a>
                </div>
                
                <div class="col-md-4 mb-4">
                    <div class="card h-100">
                        <img src="path/to/project3.jpg" class="card-img-top" alt="Project 3 Screenshot" />
                        <img src="path/to/project3_hover.jpg" class="card-hover-img" alt="Project 3 Hover" />
                        <div class="card-body">
                            <h5 class="card-title">{ "Project 3" }</h5>
                            <p class="card-text">{ "Description of Project 3." }</p>
                        </div>
                    </div>
                </div>
            </div>
            <div class="row">
                <div class="col-md-4 mb-4">
                    <div class="card h-100">
                        <img src="path/to/project4.jpg" class="card-img-top" alt="Project 4 Screenshot" />
                        <img src="path/to/project4_hover.jpg" class="card-hover-img" alt="Project 4 Hover" />
                        <div class="card-body">
                            <h5 class="card-title">{ "Project 4" }</h5>
                            <p class="card-text">{ "Description of Project 4." }</p>
                        </div>
                    </div>
                </div>
                <div class="col-md-4 mb-4">
                    <div class="card h-100">
                        <img src="path/to/project5.jpg" class="card-img-top" alt="Project 5 Screenshot" />
                        <img src="path/to/project5_hover.jpg" class="card-hover-img" alt="Project 5 Hover" />
                        <div class="card-body">
                            <h5 class="card-title">{ "Project 5" }</h5>
                            <p class="card-text">{ "Description of Project 5." }</p>
                        </div>
                    </div>
                </div>
                <div class="col-md-4 mb-4">
                    <div class="card h-100">
                        <img src="path/to/project6.jpg" class="card-img-top" alt="Project 6 Screenshot" />
                        <img src="path/to/project6_hover.jpg" class="card-hover-img" alt="Project 6 Hover" />
                        <div class="card-body">
                            <h5 class="card-title">{ "Project 6" }</h5>
                            <p class="card-text">{ "Description of Project 6." }</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        </>
    }
}