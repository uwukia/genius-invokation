macro_rules! impl_from {
    ($variant:ident : $child:ident => $parent:path) => {
        impl From<$child> for $parent {
            fn from(value: $child) -> $parent {
                <$parent>::$variant(value)
            }
        }
    };
    ($variant:ident : $child:ident => $parent:path => $higher:path) => {
        impl_from!($variant : $child => $parent);

        impl From<$child> for $higher {
            fn from(value: $child) -> $higher {
                <$higher>::from(<$parent>::from(value))
            }
        }
    };
    ($variant:ident : $child:ident => $parent:path => $higher:path => $highest:path) => {
        impl_from!($variant : $child => $parent => $higher);

        impl From<$child> for $highest {
            fn from(value: $child) -> $highest {
                <$highest>::from(<$higher>::from(<$parent>::from(value)))
            }
        }
    };
}

macro_rules! impl_match_method {
    ($($method:ident -> $return:ty { $var:ident : $match:pat },)+) => {
        $(
            fn $method(&self) -> Option<$return> {
                if let $match = self { Some(*$var) } else { None }
            }
        )+
    };
}

pub use equip::*;
mod equip;

pub use event::*;
mod event;

pub use support::*;
mod support;

pub use cost::{CardCost, DiceCost};
mod cost;

pub use card_trait::PlayingCard;
mod card_trait;

type Price = u16;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ActionCard {
    Equipment(EquipmentCard),
    Support(SupportCard),
    Event(EventCard),
}

impl PlayingCard for ActionCard {
    fn name(&self) -> &'static str {
        match self {
            Self::Equipment(card) => card.name(),
            Self::Support(card)   => card.name(),
            Self::Event(card)     => card.name(),
        }
    }

    fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Equipment(card) => card.shop_price(),
            Self::Support(card)   => card.shop_price(),
            Self::Event(card)     => card.shop_price(),
        }
    }

    fn cost(&self) -> CardCost {
        match self {
            Self::Equipment(card) => card.cost(),
            Self::Support(card)   => card.cost(),
            Self::Event(card)     => card.cost(),
        }
    }

    impl_match_method!(
        equipment    -> EquipmentCard { card: Self::Equipment(card) },
        support      -> SupportCard { card: Self::Support(card) },
        event        -> EventCard { card: Self::Event(card) },
        artifact     -> ArtifactCard { card: Self::Equipment(EquipmentCard::Artifact(card)) },
        talent       -> TalentCard { card: Self::Equipment(EquipmentCard::Talent(card)) },
        weapon       -> WeaponCard { card: Self::Equipment(EquipmentCard::Weapon(card)) },
        companion    -> CompanionCard { card: Self::Support(SupportCard::Companion(card)) },
        location     -> LocationCard { card: Self::Support(SupportCard::Location(card)) },
        item         -> ItemCard { card: Self::Support(SupportCard::Item(card)) },
        food         -> FoodCard { card: Self::Event(EventCard::Food(card)) },
        normal_event -> NormalEventCard { card: Self::Event(EventCard::Normal(card)) },
        resonance    -> ElementalResonanceCard { card: Self::Event(EventCard::Resonance(card)) },
    );
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

impl_from!(Action: ActionCard => crate::Card);