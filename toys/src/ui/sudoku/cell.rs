use dioxus::prelude::*;
use tracing::info;

use crate::ui::sudoku::SUDOKU;

#[component]
pub fn Cell(index: usize) -> Element {
    let mut show_modal = use_signal(|| false);
    let mut modal_left = use_signal(|| 0);
    let mut modal_top = use_signal(|| 0);

    let num = SUDOKU.read()[index];
    info!("Index: {index}, Sudoku number: {num}");

    rsx!(
        button { class: "btn btn-xs w-6 focus-visible:outline-blue-500",
            r#type: "button",
            tabindex: "{index + 1}",
            onclick: move|e| {
                let point = e.client_coordinates();
                if point.x as i32 == 0 || point.y as i32 == 0 {
                    return;
                }
                modal_left.set(point.x as i32);
                modal_top.set(point.y as i32);
                show_modal.set(true);
            },
            onkeydown: move|e| {
                match e.code(){
                    Code::Digit0 | Code::Numpad0 => SUDOKU.write()[index] = 0,
                    Code::Digit1 | Code::Numpad1 => SUDOKU.write()[index] = 1,
                    Code::Digit2 | Code::Numpad2 => SUDOKU.write()[index] = 2,
                    Code::Digit3 | Code::Numpad3 => SUDOKU.write()[index] = 3,
                    Code::Digit4 | Code::Numpad4 => SUDOKU.write()[index] = 4,
                    Code::Digit5 | Code::Numpad5 => SUDOKU.write()[index] = 5,
                    Code::Digit6 | Code::Numpad6 => SUDOKU.write()[index] = 6,
                    Code::Digit7 | Code::Numpad7 => SUDOKU.write()[index] = 7,
                    Code::Digit8 | Code::Numpad8 => SUDOKU.write()[index] = 8,
                    Code::Digit9 | Code::Numpad9 => SUDOKU.write()[index] = 9,
                    _ => e.stop_propagation(),
                }

                show_modal.set(false);
            },
            p{ class: if num > 0 {"opacity-100"} else {"opacity-0"},
                "{num}"
            }
        }
        dialog { class: "modal", open: show_modal,
            div { class: "modal-box p-2 w-auto flex flex-col items-center",
                position: "fixed",
                left: "{modal_left}px",
                top: "{modal_top}px",
                div {class: "grid grid-cols-3 grid-rows-3 gap-2",
                for i in 0..9{
                    button{ class: "btn btn-xs w-6",
                        onclick: move|_|{
                            show_modal.set(false);
                            SUDOKU.write()[index] = i+1;
                        },
                        "{i+1}"
                    }
                }
                }
                button{ class: "btn btn-xs btn-outline btn-error w-20 mt-2",
                    onclick: move|_|{
                        show_modal.set(false);
                        SUDOKU.write()[index] = 0;
                    },
                    "清除"
                }
            }
            div { class: "modal-backdrop",
                onclick: move|_| show_modal.set(false),
            }
        }
    )
}
