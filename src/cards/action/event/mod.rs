use crate::CardCost;

pub use normal::NormalEventCard;
mod normal;

pub use resonance::ElementalResonanceCard;
mod resonance;

pub use food::FoodCard;
mod food;

use super::{Price, PlayingCard};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum EventCard {
    /// Normal types of event cards, with no subtypes, might rename this variant and subtype later
    Normal(NormalEventCard),
    /// Elemental Resonance cards
    Resonance(ElementalResonanceCard),
    /// Food buff cards
    Food(FoodCard),
}

impl PlayingCard for EventCard {
    fn name(&self) -> &'static str {
        match self {
            Self::Normal(card)    => card.name(),
            Self::Resonance(card) => card.name(),
            Self::Food(card)      => card.name(),
        }
    }

    fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Normal(card)    => card.shop_price(),
            Self::Resonance(card) => card.shop_price(),
            Self::Food(card)      => card.shop_price(),
        }
    }

    fn cost(&self) -> CardCost {
        match self {
            Self::Normal(card)    => card.cost(),
            Self::Resonance(card) => card.cost(),
            Self::Food(card)      => card.cost(),
        }
    }

    fn event(&self) -> Option<EventCard> {
        Some(*self)
    }

    impl_match_method!(
        normal_event -> NormalEventCard { card: Self::Normal(card) },
        resonance    -> ElementalResonanceCard { card: Self::Resonance(card) },
        food         -> FoodCard { card: Self::Food(card) },
    );
}

use std::cmp::Ordering;
use super::CardOrd;
impl CardOrd for EventCard {
    fn cmp(&self, other: &Self) -> Ordering {
        // Resonance < Normal < Food
        match (self, other) {
            (Self::Resonance(x), Self::Resonance(y)) => x.cmp(y),
            (Self::Normal(x), Self::Normal(y))       => x.cmp(y),
            (Self::Food(x), Self::Food(y))           => x.cmp(y),
            (Self::Resonance(_), _) => Ordering::Less,
            (_, Self::Resonance(_)) => Ordering::Greater,
            (Self::Normal(_), _)    => Ordering::Less,
            (_, Self::Normal(_))    => Ordering::Greater,
        }
    }
}

impl_from!(Event: EventCard => crate::ActionCard => crate::Card);