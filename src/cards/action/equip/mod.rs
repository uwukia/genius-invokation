pub use artifact::ArtifactCard;
mod artifact;

pub use weapon::WeaponCard;
mod weapon;

pub use talent::TalentCard;
mod talent;

use crate::CardCost;
use super::{Price, PlayingCard};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentCard {
    Talent(TalentCard),
    Weapon(WeaponCard),
    Artifact(ArtifactCard),
}

impl PlayingCard for EquipmentCard {
    fn name(&self) -> &'static str {
        match self {
            Self::Talent(card)   => card.name(),
            Self::Weapon(card)   => card.name(),
            Self::Artifact(card) => card.name(),
        }
    }

    fn shop_price(&self) -> Option<Price> {
        match self {
            Self::Talent(_)      => None,
            Self::Weapon(card)   => card.shop_price(),
            Self::Artifact(card) => card.shop_price(),
        }
    }

    fn cost(&self) -> CardCost {
        match self {
            Self::Talent(card)   => card.cost(),
            Self::Weapon(card)   => card.cost(),
            Self::Artifact(card) => card.cost(),
        }
    }

    fn equipment(&self) -> Option<EquipmentCard> {
        Some(*self)
    }

    impl_match_method!(
        artifact -> ArtifactCard { card: Self::Artifact(card) },
        talent   -> TalentCard { card: Self::Talent(card) },
        weapon   -> WeaponCard { card: Self::Weapon(card) },
    );
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

impl_from!(Equipment: EquipmentCard => crate::ActionCard => crate::Card);