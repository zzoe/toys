use dioxus::prelude::*;

use crate::service::Api;

mod cell;

pub static SUDOKU: GlobalSignal<[u16; 81]> = Signal::global(|| [0; 81]);

#[component]
pub fn Sudoku() -> Element {
    let mut last_sudoku = use_signal(|| [0; 81]);
    let api = use_coroutine_handle::<Api>();

    rsx!(
        div { class: "flex flex-row overflow-x-auto gap-3",
            table { class:"table-sm size-fit", style: "border: 1px solid",
                tbody {
                    for i in 0..9{
                        tr { class: "hover",
                            style: if i ==3 || i==6 {"border-top: 1px solid"} else {""},
                            for j in 0..9{
                                td { class:"p-2",
                                    style: if j ==3 || j==6 {"border-left: 1px solid"} else {""},
                                    onmouseenter: |_| (),
                                    cell::Cell{
                                        index: 9*i+j
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div{ class:"flex flex-col gap-3 w-32",
                button{ class:"btn btn-success",
                    onclick: move|_| {
                        let sudoku = SUDOKU.read();
                        last_sudoku.set(*sudoku);
                        api.send(Api::Sudoku(*sudoku));
                    },
                    "计算"
                }
                button{ class:"btn btn-outline btn-error",
                    onclick: move|_| {
                        *SUDOKU.write() = *last_sudoku.read();
                        last_sudoku.set([0;81]);
                    },
                    "重置"
                }
            }

        }
    )
}
