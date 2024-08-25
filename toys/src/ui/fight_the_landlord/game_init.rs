use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Card, Hand, SuitCard, DECK_OF_CARDS};

use crate::ui::fight_the_landlord::card::CardUI;
use crate::ui::fight_the_landlord::{OurHand, PlayerRole, RemainHand};
use crate::ui::Route;

// 当前操作对象
#[derive(Copy, Clone, Debug, PartialEq)]
enum OperatingObject {
    // 地主底牌
    LandlordHand,
    // 自己手牌
    OurHand,
}

#[component]
pub fn FTLInit() -> Element {
    let nav = navigator();
    let mut remain_hand = use_context::<Signal<RemainHand>>();
    let mut our_hand = use_context::<Signal<OurHand>>();
    let mut player_role = use_context::<Signal<PlayerRole>>();
    let mut landlord_hand = use_signal(Hand::default);
    // 当前操作对象
    let mut operating_object = use_signal(|| OperatingObject::LandlordHand);

    // 抓牌
    let draw_card = |card: SuitCard| {
        move |_: MouseEvent| {
            // 操作地主底牌
            if operating_object() == OperatingObject::LandlordHand {
                let mut hand1 = landlord_hand.write();
                let mut hand2 = our_hand.write();
                if hand1.size() > 2 || hand2.0.size() > 19 {
                    return;
                }

                hand1.insert_suit_card(card);
                if player_role() == PlayerRole::Landlord {
                    hand2.0.insert_suit_card(card);
                }
            } else {
                let mut hand2 = our_hand.write();
                let mut size = hand2.0.size();
                if player_role() == PlayerRole::Landlord {
                    size -= landlord_hand.read().size();
                }

                if size > 16 {
                    return;
                }
                hand2.0.insert_suit_card(card);
            }

            remain_hand.write().0.remove_suit_card(card);
        }
    };

    // 放回手牌
    let put_back = |card: SuitCard| {
        move |_: MouseEvent| {
            landlord_hand.write().remove_suit_card(card);
            our_hand.write().0.remove_suit_card(card);
            remain_hand.write().0.insert_suit_card(card);
        }
    };

    let remain_cards = DECK_OF_CARDS.map(|suit_card| {
        let key = format!("r{}", u64::from(suit_card));
        rsx!(
            div{ class: if suit_card == SuitCard::Spades(Card::Two) {"row-start-1"} else {""},
                key: "remain-{key}",
                CardUI {
                    suit_card,
                    containing: remain_hand.read().0.contains(suit_card),
                    on_click: draw_card(suit_card),
                }
            }
        )
    });

    let landlord_cards = landlord_hand.read().map(|suit_card: SuitCard| {
        let key = format!("l{}", u64::from(suit_card));
        rsx!(CardUI {
            key: "landlord-{key}",
            suit_card,
            containing: true,
            on_click: put_back(suit_card),
        })
    });

    let our_cards = our_hand.read().0.map(|suit_card: SuitCard| {
        let key = format!("o{}", u64::from(suit_card));
        rsx!(CardUI {
            key: "our-{key}",
            suit_card,
            containing: true,
            on_click: put_back(suit_card),
        })
    });

    let landlord_hand_outline = (*operating_object.read() == OperatingObject::LandlordHand)
        .then_some("outline-blue-400")
        .unwrap_or_default();
    let our_hand_outline = (*operating_object.read() == OperatingObject::OurHand)
        .then_some("outline-blue-400")
        .unwrap_or_default();

    rsx!(
        div{ class:"flex flex-col space-y-3",
            div{ class:"grid grid-flow-col grid-rows-4 grid-cols-14 w-fit bg-blue-100 pr-2 pb-2",
                {remain_cards},
            }
            div{ class: "flex flex-row",
                label { class: "label",
                    span{ class: "label-text", "选择地主："}
                }
                label { class: "label cursor-pointer",
                    input { class: "radio radio-success", r#type: "radio", name: "radio-1", checked: true,
                        onclick: move|_| {
                            *player_role.write() = PlayerRole::NextFarmer;
                            landlord_hand.read().for_each(|s|{
                                our_hand.write().0.remove_suit_card(s);
                            });
                        },
                    }
                    span { class: "label-text ml-1", "上家" }
                }
                label { class: "label cursor-pointer ml-2",
                    input { class: "radio radio-success", r#type: "radio", name: "radio-1",
                        onclick: move|_| {
                            *player_role.write() = PlayerRole::Landlord;
                            landlord_hand.read().for_each(|s|{
                                our_hand.write().0.insert_suit_card(s);
                            });
                        },
                    }
                    span { class: "label-text ml-1", "自己" }
                }
                label { class: "label cursor-pointer ml-2",
                    input { class: "radio radio-success", r#type: "radio", name: "radio-1",
                        onclick: move|_| {
                            *player_role.write() = PlayerRole::LastFarmer;
                            landlord_hand.read().for_each(|s|{
                                our_hand.write().0.remove_suit_card(s);
                            });
                        },
                    }
                    span { class: "label-text ml-1", "下家" }
                }
            }
            div{ class: "flex flex-row min-h-16 whitespace-nowrap",
                onclick: move|_| *operating_object.write() = OperatingObject::LandlordHand,
                label { class: "label min-w-32",
                    span{ class: "label-text", "选择底牌："}
                }
                div{ class: "flex flex-wrap shadow grow-0 min-w-44 w-fit h-full pr-2 pb-2 justify-center rounded-xl
                outline-none {landlord_hand_outline} hover:outline-blue-400 bg-blue-100",
                    {landlord_cards},
                }
            }
            div{ class: "flex flex-row min-h-16 w-full whitespace-nowrap",
                onclick: move|_| *operating_object.write() = OperatingObject::OurHand,
                label { class: "label min-w-32",
                    span{ class: "label-text", "选择自己的手牌："}
                }
                div{ class: "flex flex-wrap shadow grow-0 w-full max-w-222 h-full pr-2 pb-2 justify-center rounded-xl
                outline-none {our_hand_outline} hover:outline-blue-400 bg-blue-100",
                    {our_cards},
                }
            }
            div{ class: "flex flex-row justify-evenly",
                button{class:"btn btn-wide btn-outline btn-secondary", "重置"}
                button{class:"btn btn-wide btn-outline btn-primary",
                    onclick: move|_| {nav.push(Route::FTLPlay{});},
                    "开始"
                }
            }
        }
    )
}
