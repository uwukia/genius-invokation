pub use character::*;
mod character;

pub use action::*;
mod action;

#[cfg(feature = "deck-url")]
pub use deck_url::{deck_from_url, UrlDeckError};

#[cfg(feature = "deck-url")]
mod deck_url;

/// Represents any card in Genius Invokation TCG
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Character(CharacterCard),
    Action(ActionCard),
}

impl Card {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Character(card) => card.name(),
            Self::Action(card)    => card.name(),
        }
    }
}

/// This is so that decks that include the same cards are also considered to be equal to each other
/// 
/// Array of cards is sorted before being built into a `Deck`.
/// 
/// However, I don't want to implement `Ord` as this is not supposed to be stable, and in an API
/// perspective, it's not idiomatic for cards to implement it publicly
pub(crate) trait CardOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering;
}
