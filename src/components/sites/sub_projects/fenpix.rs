use fenpix::{fen_to_board_buffer, ChessAssets};
use yew::prelude::*;
use web_sys::{HtmlInputElement, SubmitEvent};
use base64::encode;

fn is_valid_fen(fen: &str) -> bool {
    if fen.is_empty() {
        web_sys::console::error_1(&"Please enter a FEN, like 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b'".into());
        return false;
    }
    let fields: Vec<&str> = fen.split(' ').collect();
    if fields.len() < 2 {
        web_sys::console::error_1(&"FEN must have at least 2 fields (ranks + side to move). E.g.: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b'".into());
        return false;
    }
    let ranks: Vec<&str> = fields[0].split('/').collect();
    if ranks.len() != 8 {
        web_sys::console::error_1(&"FEN must have 8 ranks. E.g.: 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b'".into());
        return false;
    }
    true
}

#[function_component(Fenpix)]
pub fn fenpix() -> Html {
    let fen_input_ref = use_node_ref();
    let image_data = use_state(|| None);

    let on_submit = {
        let fen_input_ref = fen_input_ref.clone();
        let image_data = image_data.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(input) = fen_input_ref.cast::<HtmlInputElement>() {
                let fen = input.value();
                if !is_valid_fen(&fen) {
                    return;
                }

                let chess_assets = ChessAssets::default();
                match fen_to_board_buffer(&fen, 4, &chess_assets) {
                    Ok(buffer) => {
                        let base64_image = encode(&buffer);
                        image_data.set(Some(base64_image));
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Error generating board: {:?}", err).into());
                    }
                }
            }
        })
    };

    html! {
        <div class="container mt-5">
            <h1 class="display-4">{ "Fenpix" }</h1>
            <p class="lead">{"Enter a FEN like 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b' to generate a chessboard"}</p>
            <form onsubmit={on_submit}>
                <div class="mb-3">
                    <label for="fenInput" class="form-label">{ "FEN String" }</label>
                    <input type="text" class="form-control" id="fenInput" ref={fen_input_ref} placeholder="rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b" />
                </div>
                <button type="submit" class="btn btn-primary">{ "Generate Image" }</button>
            </form>
            {
                if let Some(image_data) = &*image_data {
                    html! {
                        <div class="mt-4">
                            <img src={format!("data:image/png;base64,{}", image_data)} alt="Chess Board" />
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}