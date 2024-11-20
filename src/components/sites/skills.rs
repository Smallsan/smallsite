use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    html! {
        <>
        <style>
        {"

        .carousel-container {
            overflow: hidden;
            width: 100%;
            position: relative;
        }
        
        .carousel-content {
            display: flex;
            animation: scroll 20s linear infinite;
        }
        
        .card {
            min-width: 300px;
            margin-right: 20px;
            color: white;
            border: none;
            border-radius: 10px;
            transition: transform 0.3s;
            padding-top: 20px; /* Add padding to the top */
        }

        .card:hover {
            transform: scale(1.05);
        }

        .card-body {
            padding: 20px;
        }

        .react { background-color: #61dafb; }
        .vue { background-color: #42b883; }
        .bootstrap { background-color: #563d7c; }
        .jquery { background-color: #0769ad; }
        .rocket { background-color: #de4c4f; }
        .tauri { background-color: #ffc107; }
        .pytorch { background-color: #ee4c2c; }

        .progress {
            height: 20px;
            background-color: #e9ecef;
            border-radius: 10px;
            overflow: hidden;
        }

        .progress-bar {
            height: 100%;
            background: linear-gradient(90deg, #4caf50, #8bc34a);
            border-radius: 10px;
        }

        .skill-icon {
            margin-right: 10px;
        }

        .badge {
            background-color: #007bff;
            color: white;
            padding: 5px 10px;
            border-radius: 5px;
            font-size: 0.9em;
            margin-left: 10px; /* Add space between skill name and rank */
        }

        .framework-icon {
            width: 3em;
            height: 3em;
        }

        @keyframes scroll {
            0% {
                transform: translateX(0);
            }
            100% {
                transform: translateX(-100%);
            }
        }
        "}
        </style>
        
        <div class="container mt-5">
            <h2 class="mb-4">{ "Skills" }</h2>
            <div class="row">
                <div class="col-md-6">
                    <h3>{ "Programming Languages" }</h3>

                    <div class="mb-3">
                        <label for="rust" class="form-label">
                            <i class="fab fa-rust skill-icon"></i>{ "Rust" }
                            <span class="badge">{ "Expert" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 100%;" aria-valuenow="100" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="java" class="form-label">
                            <i class="fab fa-java skill-icon"></i>{ "Java" }
                            <span class="badge">{ "Advanced" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 80%;" aria-valuenow="80" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="javascript" class="form-label">
                            <i class="fab fa-js skill-icon"></i>{ "JavaScript" }
                            <span class="badge">{ "Intermediate" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 70%;" aria-valuenow="70" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>
                    <div class="mb-3">
                        <label for="python" class="form-label">
                            <i class="fab fa-python skill-icon"></i>{ "Python" }
                            <span class="badge">{ "Intermediate" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 60%;" aria-valuenow="60" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                </div>
                <div class="col-md-6">
                    <h3>{ "Other Skills" }</h3>

                    <div class="mb-3">
                        <label for="mysql" class="form-label">
                            <i class="fas fa-database skill-icon"></i>{ "MYSQL" }
                            <span class="badge">{ "Advanced" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 80%;" aria-valuenow="80" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="html" class="form-label">
                            <i class="fab fa-html5 skill-icon"></i>{ "HTML" }
                            <span class="badge">{ "Intermediate" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 70%;" aria-valuenow="70" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="css" class="form-label">
                            <i class="fab fa-css3-alt skill-icon"></i>{ "CSS" }
                            <span class="badge">{ "Intermediate" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 70%;" aria-valuenow="70" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                    <div class="mb-3">
                        <label for="php" class="form-label">
                            <i class="fab fa-php skill-icon"></i>{ "PHP" }
                            <span class="badge">{ "Beginner" }</span>
                        </label>
                        <div class="progress">
                            <div class="progress-bar" role="progressbar" style="width: 50%;" aria-valuenow="50" aria-valuemin="0" aria-valuemax="100"></div>
                        </div>
                    </div>

                </div>
            </div>

            <div class="row mt-5">
                <div class="col-md-12">
                    <h3>{ "Frameworks" }</h3>
                    <div class="carousel-container">
                        <div class="carousel-content">
                            <div class="card mb-4 text-center react">
                                <i class="fab fa-react fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "React" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using React." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center vue">
                                <i class="fab fa-vuejs fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Vue" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Vue." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center bootstrap">
                                <i class="fab fa-bootstrap fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Bootstrap" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Bootstrap." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center jquery">
                                <i class="fab fa-js-square fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Jquery" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Jquery." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center rocket">
                                <i class="fas fa-rocket fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Rocket" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Rocket." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center tauri">
                                <i class="fas fa-cogs fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Tauri" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Tauri." }</p>
                                </div>
                            </div>
                            <div class="card mb-4 text-center pytorch">
                                <i class="fas fa-brain fa-3x"></i>
                                <div class="card-body">
                                    <h5 class="card-title">{ "Pytorch" }</h5>
                                    <p class="card-text">{ "Experience with building web applications using Pytorch." }</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        </>
    }
}