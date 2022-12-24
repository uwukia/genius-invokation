use crate::CardCost;

pub use companion::CompanionCard;
mod companion;

pub use location::LocationCard;
mod location;

pub use item::ItemCard;
mod item;

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum SupportCard {
    Companion(CompanionCard),
    Location(LocationCard),
    Item(ItemCard),
}

impl SupportCard {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Companion(card) => card.name(),
            Self::Location(card)  => card.name(),
            Self::Item(card)      => card.name(),
        }
    }

    /// Returns `Some(price)` in Lucky Coins to buy the given card in the shop.
    /// 
    /// Cards not sold in the shop (such as [`Paimon`]) return `None`
    /// 
    /// [`Paimon`]: CompanionCard::Paimon
    pub fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Companion(card) => card.shop_price(),
            Self::Location(card)  => Some(card.shop_price()),
            Self::Item(card)      => Some(card.shop_price()),
        }
    }

    pub fn cost(&self) -> CardCost {
        match self {
            Self::Companion(card) => card.cost(),
            Self::Location(card)  => card.cost(),
            Self::Item(card)      => card.cost(),
        }
    }
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