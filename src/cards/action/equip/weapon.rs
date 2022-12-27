use crate::{CardCost, WeaponType::{self, *}};
use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum WeaponCard {
    MagicGuide,
    SacrificialFragments,
    SkywardAtlas,
    RavenBow,
    SacrificialBow,
    SkywardHarp,
    WhiteIronGreatsword,
    SacrificialGreatsword,
    WolfsGravestone,
    WhiteTassel,
    LithicSpear,
    SkywardSpine,
    TravelersHandySword,
    SacrificialSword,
    AquilaFavonia,
}

impl super::PlayingCard for WeaponCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        Some(self.info_dump().1)
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn weapon(&self) -> Option<WeaponCard> {
        Some(*self)
    }
}

impl WeaponCard {
    /// Retrieves the weapon type for this card
    /// 
    /// Currently, this method never returns `None`, but since some character cards have the
    /// "Other Weapons" subtype, this could potentially mean there will be cards in the future
    /// that don't fit into any of the weapon types, which would return None for this method
    pub fn subtype(&self) -> Option<WeaponType> {
        Some(self.info_dump().3)
    }

    fn info_dump(&self) -> (&'static str, Price, CardCost, WeaponType) {
        match self {
            Self::MagicGuide            => ("Magic Guide",             500, CardCost::MATCH2, Catalyst),
            Self::SacrificialFragments  => ("Sacrificial Fragments",   700, CardCost::MATCH3, Catalyst),
            Self::SkywardAtlas          => ("Skyward Atlas",          1000, CardCost::MATCH3, Catalyst),
            Self::RavenBow              => ("Raven Bow",               500, CardCost::MATCH2, Bow),
            Self::SacrificialBow        => ("Sacrificial Bow",         700, CardCost::MATCH3, Bow),
            Self::SkywardHarp           => ("Skyward Harp",           1000, CardCost::MATCH3, Bow),
            Self::WhiteIronGreatsword   => ("White Iron Greatsword",   500, CardCost::MATCH2, Claymore),
            Self::SacrificialGreatsword => ("Sacrificial Greatsword",  700, CardCost::MATCH3, Claymore),
            Self::WolfsGravestone       => ("Wolf's Gravestone",      1000, CardCost::MATCH3, Claymore),
            Self::WhiteTassel           => ("White Tassel",            500, CardCost::MATCH2, Polearm),
            Self::LithicSpear           => ("Lithic Spear",            700, CardCost::MATCH3, Polearm),
            Self::SkywardSpine          => ("Skyward Spine",          1000, CardCost::MATCH3, Polearm),
            Self::TravelersHandySword   => ("Traveler's Handy Sword",  500, CardCost::MATCH2, Sword),
            Self::SacrificialSword      => ("Sacrificial Sword",       700, CardCost::MATCH3, Sword),
            Self::AquilaFavonia         => ("Aquila Favonia",         1000, CardCost::MATCH3, Sword),
        }
    }
}

impl super::CardOrd for WeaponCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Weapon: WeaponCard => crate::EquipmentCard => crate::ActionCard => crate::Card);