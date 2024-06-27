use dioxus::prelude::*;
use poker::{Card, SuitCard};

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
