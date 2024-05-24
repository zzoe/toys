use dioxus::prelude::*;
use tracing::info;

pub fn Sudoku() -> Element {
    info!("Sudoku");
    rsx!(article {
        h1{
            "Sudoku"
        }
    })
}
