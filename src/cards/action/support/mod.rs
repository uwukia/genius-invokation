use crate::CardCost;

pub use companion::CompanionCard;
mod companion;

pub use location::LocationCard;
mod location;

pub use item::ItemCard;
mod item;

use super::{Price, PlayingCard};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum SupportCard {
    Companion(CompanionCard),
    Location(LocationCard),
    Item(ItemCard),
}

impl PlayingCard for SupportCard {
    fn name(&self) -> &'static str {
        match self {
            Self::Companion(card) => card.name(),
            Self::Location(card)  => card.name(),
            Self::Item(card)      => card.name(),
        }
    }

    fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Companion(card) => card.shop_price(),
            Self::Location(card)  => card.shop_price(),
            Self::Item(card)      => card.shop_price(),
        }
    }

    fn cost(&self) -> CardCost {
        match self {
            Self::Companion(card) => card.cost(),
            Self::Location(card)  => card.cost(),
            Self::Item(card)      => card.cost(),
        }
    }

    fn support(&self) -> Option<SupportCard> {
        Some(*self)
    }

    impl_match_method!(
        companion -> CompanionCard { card: Self::Companion(card) },
        location  -> LocationCard { card: Self::Location(card) },
        item -> ItemCard { card: Self::Item(card) },
    );
}

use std::cmp::Ordering;
use super::CardOrd;
impl CardOrd for SupportCard {
    fn cmp(&self, other: &Self) -> Ordering {
        // Location < Companion < Item
        match (self, other) {
            (Self::Location(x), Self::Location(y))   => x.cmp(y),
            (Self::Companion(x), Self::Companion(y)) => x.cmp(y),
            (Self::Item(x), Self::Item(y))           => x.cmp(y),
            (Self::Location(_), _)  => Ordering::Less,
            (_, Self::Location(_))  => Ordering::Greater,
            (Self::Companion(_), _) => Ordering::Less,
            (_, Self::Companion(_)) => Ordering::Greater,
        }
    }
}

impl_from!(Support: SupportCard => crate::ActionCard => crate::Card);