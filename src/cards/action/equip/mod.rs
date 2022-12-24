pub use artifact::ArtifactCard;
mod artifact;

pub use weapon::WeaponCard;
mod weapon;

pub use talent::TalentCard;
mod talent;

use crate::CardCost;
use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentCard {
    Talent(TalentCard),
    Weapon(WeaponCard),
    Artifact(ArtifactCard),
}

impl EquipmentCard {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Talent(card)   => card.name(),
            Self::Weapon(card)   => card.name(),
            Self::Artifact(card) => card.name(),
        }
    }

    /// Returns `Some(price)` in Lucky Coins to buy the given card in the shop.
    /// 
    /// Cards not sold in the shop (such as [talent] cards) return `None`
    /// 
    /// [talent]: TalentCard
    pub fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Talent(_)      => None,
            Self::Weapon(card)   => Some(card.shop_price()),
            Self::Artifact(card) => Some(card.shop_price()),
        }
    }

    pub fn cost(&self) -> CardCost {
        match self {
            Self::Talent(card)   => card.cost(),
            Self::Weapon(card)   => card.cost(),
            Self::Artifact(card) => card.cost(),
        }
    }
}

use std::cmp::Ordering;
use super::CardOrd;
impl CardOrd for EquipmentCard {
    fn cmp(&self, other: &Self) -> Ordering {
        // Talent < Weapon < Artifact
        match (self, other) {
            (Self::Talent(x), Self::Talent(y))     => x.cmp(y),
            (Self::Weapon(x), Self::Weapon(y))     => x.cmp(y),
            (Self::Artifact(x), Self::Artifact(y)) => x.cmp(y),
            (Self::Talent(_), _) => Ordering::Less,
            (_, Self::Talent(_)) => Ordering::Greater,
            (Self::Weapon(_), _) => Ordering::Less,
            (_, Self::Weapon(_)) => Ordering::Greater,
        }
    }
}