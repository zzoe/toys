use dioxus::prelude::*;
use dioxus_router::prelude::*;
use poker::{Hand, DECK_OF_CARDS};

use crate::ui::Route;

pub use game_init::FTLInit;
pub use game_play::FTLPlay;

mod card;
mod game_init;
mod game_play;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct RemainHand(Hand);
#[derive(Default, Copy, Clone, Debug, PartialEq)]
struct OurHand(Hand);

#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerRole {
    LastFarmer,
    Landlord,
    NextFarmer,
}

#[component]
pub fn FightTheLandlord() -> Element {
    use_context_provider(|| Signal::new(RemainHand(DECK_OF_CARDS)));
    use_context_provider(|| Signal::new(OurHand(Hand::default())));
    use_context_provider(|| Signal::new(PlayerRole::NextFarmer));

    rsx!(Outlet::<Route> {})
}
