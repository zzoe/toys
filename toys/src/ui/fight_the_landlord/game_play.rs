use crate::ui::fight_the_landlord::card::CardUI;
use crate::ui::fight_the_landlord::RemainHand;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Card, Hand, SuitCard, DECK_OF_CARDS};

// 当前操作对象
#[derive(Copy, Clone, Debug, PartialEq)]
enum CurrentTurn {
    // 上家
    Previous,
    // 自己
    Myself,
    // 下家
    Next,
}

impl CurrentTurn {
    fn next(&mut self) {
        match self {
            CurrentTurn::Previous => *self = CurrentTurn::Myself,
            CurrentTurn::Myself => *self = CurrentTurn::Next,
            CurrentTurn::Next => *self = CurrentTurn::Previous,
        }
    }
}

#[component]
pub fn FTLPlay() -> Element {
    let nav = navigator();
    let mut remain_hand = use_context::<Signal<RemainHand>>();
    let mut previous_hand = use_signal(Hand::default);
    let mut next_hand = use_signal(Hand::default);
    // 当前操作对象
    let mut operating_object = use_signal(|| CurrentTurn::Previous);

    let remain_cards = DECK_OF_CARDS.map(|suit_card| {
        let key = format!("r{}", u64::from(suit_card));
        rsx!(
            div{ class: if suit_card == SuitCard::Spades(Card::Two) {"row-start-1"} else {""},
                key: "remain-{key}",
                CardUI {
                    suit_card,
                    containing: remain_hand.read().0.contains(suit_card),
                    on_click: |_|(),
                }
            }
        )
    });
    let previous_cards = previous_hand.read().map(|suit_card: SuitCard| {
        let key = format!("l{}", u64::from(suit_card));
        rsx!(CardUI {
            key: "previous-{key}",
            suit_card,
            containing: true,
        })
    });
    let next_cards = next_hand.read().map(|suit_card: SuitCard| {
        let key = format!("l{}", u64::from(suit_card));
        rsx!(CardUI {
            key: "next-{key}",
            suit_card,
            containing: true,
        })
    });

    let previous_hand_outline = (*operating_object.read() == CurrentTurn::Previous)
        .then_some("outline-blue-400")
        .unwrap_or_default();
    let next_hand_outline = (*operating_object.read() == CurrentTurn::Next)
        .then_some("outline-blue-400")
        .unwrap_or_default();

    rsx!(
        div { class: "flex flex-col space-y-3 w-fit",
            div{ class: "flex flex-row items-center justify-evenly",
                label { class: "label",
                    span{ class: "label-text", "当前回合："}
                    div{ class:"bg-yellow-200",
                        p{"自己按照AI出牌"}
                    }
                }
                button{ class:"btn btn-outline btn-success w-20 h-8 min-h-8 ml-4",
                    "下一步"
                }
                button{ class:"btn btn-outline btn-secondary w-20 h-8 min-h-8 ml-4",
                    onclick: move|_|nav.go_back(),
                    "退出"
                }
            }
            div{ class:"grid grid-flow-col grid-rows-4 grid-cols-14 w-fit bg-blue-100 pr-2 pb-2",
                {remain_cards},
            }
            div{ class:"flex flex-row justify-between",
                // 地主上家
                div{ class:"grid grid-cols-6 w-fit min-h-16",
                    div{ class: "flex flex-wrap shadow grow-0 min-w-44 w-fit h-full pr-2 pb-2 justify-center rounded-xl outline-none {previous_hand_outline} hover:outline-blue-400 bg-blue-100",
                        {previous_cards},
                    }
                }
                // 地主下家
                div{ class:"grid grid-cols-6 w-fit min-h-16",
                    div{ class: "flex flex-wrap shadow grow-0 min-w-44 w-fit h-full pr-2 pb-2 justify-center rounded-xl outline-none {next_hand_outline} hover:outline-blue-400 bg-blue-100",
                        {next_cards},
                    }
                }
            }
        }
        div{ class:"flex flex-col",
            div{ class: "flex flex-row items-center",
                label { class: "label w-16",
                    span{ class: "label-text", "地主"}
                }
                div{ class:"bg-blue-100",
                    p{"123"}
                }
            }
            div{ class: "flex flex-row items-center",
                label { class: "label w-16",
                    div{ class:"flex flex-col",
                        span{ class: "label-text", "农民"}
                        span{ class: "label-text text-xs", "地主下家"}
                    }
                }
                div{ class:"bg-blue-100",
                    p{"123"}
                }
            }
            div{ class: "flex flex-row items-center",
                label { class: "label w-16",
                    div{ class:"flex flex-col",
                        span{ class: "label-text", "农民"}
                        span{ class: "label-text text-xs", "地主上家"}
                    }
                }
                div{ class:"bg-blue-100",
                    p{"123"}
                }
            }
        }
    )
}
