use crate::ui::fight_the_landlord::RemainHand;
use dioxus::prelude::*;
use poker::{Card, SuitCard, DECK_OF_CARDS};

// Remember: Owned props must implement `PartialEq`!
#[derive(Clone, PartialEq, Props)]
pub struct CardProps {
    pub suit_card: SuitCard,
    #[props(default = true)]
    containing: bool,
    pub on_click: Option<EventHandler<MouseEvent>>,
}

#[component]
pub fn CardUI(props: CardProps) -> Element {
    let show = |card: Card| match card {
        Card::Ten => "10".to_owned(),
        Card::BlackJoker => "王".to_owned(),
        Card::RedJoker => "王".to_owned(),
        c => c.to_string(),
    };

    // ♠♥♣♦
    let (suit, color, card, card_font) = match props.suit_card {
        SuitCard::Spades(c) => ("♠", "text-black", show(c), "Consolas"),
        SuitCard::Hearts(c) => ("♥", "text-red-500", show(c), "Consolas"),
        SuitCard::Clubs(c) => ("♣", "text-black", show(c), "Consolas"),
        SuitCard::Diamonds(c) if c == Card::RedJoker => ("", "text-red-500", show(c), "楷体"),
        SuitCard::Diamonds(c) if c == Card::BlackJoker => ("", "text-black", show(c), "楷体"),
        SuitCard::Diamonds(c) => ("♦", "text-red-500", show(c), "Consolas"),
    };

    let bg = (!props.containing)
        .then_some("bg-stone-300")
        .unwrap_or_default();

    rsx! {
        div {
            class: "flex relative shadow justify-center items-center ml-2 mt-2 w-9 h-11 text-2xl cursor-default select-none outline outline-amber-200 hover:bg-teal-300 {color} {bg}",
            style: " font-family: {card_font}",
            onclick: move |event| {
                if let Some(on_click) = props.on_click.as_ref() {
                    if props.containing{
                        on_click.call(event)
                    }
                }
            },
            div { class: "absolute top-0 left-0 text-sm {color}", "{suit}" }
            div { class: "absolute bottom-0 right-0 text-sm {color}", "{suit}" }
            "{card}"
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct RemainHandProps {
    pub card_handler: fn(suit_card: SuitCard) -> EventHandler<MouseEvent>,
}

#[component]
pub fn RemainHandUI(props: RemainHandProps) -> Element {
    let remain_hand = use_context::<Signal<RemainHand>>();

    let remain_cards = DECK_OF_CARDS.map(|suit_card| {
        let key = format!("r{}", u64::from(suit_card));
        rsx!(
            div{ class: if suit_card == SuitCard::Spades(Card::Two) {"row-start-1"} else {""},
                key: "remain-{key}",
                CardUI {
                    suit_card,
                    containing: remain_hand.read().0.contains(suit_card),
                    on_click: (props.card_handler)(suit_card),
                }
            }
        )
    });

    rsx!(
        div{ class:"grid grid-flow-col grid-rows-4 grid-cols-14 w-fit bg-blue-100 pr-2 pb-2",
            {remain_cards},
        }
    )
}
