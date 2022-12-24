pub use equip::*;
mod equip;

pub use event::*;
mod event;

pub use support::*;
mod support;

pub use cost::{CardCost, DiceCost};
mod cost;

type Price = u16;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ActionCard {
    Equipment(EquipmentCard),
    Support(SupportCard),
    Event(EventCard),
}

impl ActionCard {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Equipment(card) => card.name(),
            Self::Support(card)   => card.name(),
            Self::Event(card)     => card.name(),
        }
    }

    /// Returns `Some(price)` in Lucky Coins to buy the given card in the shop.
    /// 
    /// Cards not sold in the shop (such as [talent] cards or [`Paimon`]) return `None`
    /// 
    /// [talent]: TalentCard
    /// [`Paimon`]: CompanionCard::Paimon
    pub fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Equipment(card) => card.shop_price(),
            Self::Support(card)   => card.shop_price(),
            Self::Event(card)     => card.shop_price(),
        }
    }

    pub fn cost(&self) -> CardCost {
        match self {
            Self::Equipment(card) => card.cost(),
            Self::Support(card)   => card.cost(),
            Self::Event(card)     => card.cost(),
        }
    }
}

use std::cmp::Ordering;
use super::CardOrd;
impl CardOrd for ActionCard {
    fn cmp(&self, other: &Self) -> Ordering {
        // Equipment < Support < Event
        match (self, other) {
            (Self::Equipment(x), Self::Equipment(y)) => x.cmp(y),
            (Self::Support(x), Self::Support(y))     => x.cmp(y),
            (Self::Event(x), Self::Event(y))         => x.cmp(y),
            (Self::Equipment(_), _) => Ordering::Less,
            (_, Self::Equipment(_)) => Ordering::Greater,
            (Self::Support(_), _)   => Ordering::Less,
            (_, Self::Support(_))   => Ordering::Greater,
        }
    }
}