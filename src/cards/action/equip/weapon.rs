use crate::{CardCost, DiceCost::Same, WeaponType::{self, *}};
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

impl WeaponCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn shop_price(&self) -> Price {
        self.info_dump().1
    }

    pub fn cost(&self) -> CardCost {
        self.info_dump().2
    }

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
            Self::MagicGuide            => ("Magic Guide",             500, CardCost::new(Same, 2, 0), Catalyst),
            Self::SacrificialFragments  => ("Sacrificial Fragments",   700, CardCost::new(Same, 3, 0), Catalyst),
            Self::SkywardAtlas          => ("Skyward Atlas",          1000, CardCost::new(Same, 3, 0), Catalyst),
            Self::RavenBow              => ("Raven Bow",               500, CardCost::new(Same, 2, 0), Bow),
            Self::SacrificialBow        => ("Sacrificial Bow",         700, CardCost::new(Same, 3, 0), Bow),
            Self::SkywardHarp           => ("Skyward Harp",           1000, CardCost::new(Same, 3, 0), Bow),
            Self::WhiteIronGreatsword   => ("White Iron Greatsword",   500, CardCost::new(Same, 2, 0), Claymore),
            Self::SacrificialGreatsword => ("Sacrificial Greatsword",  700, CardCost::new(Same, 3, 0), Claymore),
            Self::WolfsGravestone       => ("Wolf's Gravestone",      1000, CardCost::new(Same, 3, 0), Claymore),
            Self::WhiteTassel           => ("White Tassel",            500, CardCost::new(Same, 2, 0), Polearm),
            Self::LithicSpear           => ("Lithic Spear",            700, CardCost::new(Same, 3, 0), Polearm),
            Self::SkywardSpine          => ("Skyward Spine",          1000, CardCost::new(Same, 3, 0), Polearm),
            Self::TravelersHandySword   => ("Traveler's Handy Sword",  500, CardCost::new(Same, 2, 0), Sword),
            Self::SacrificialSword      => ("Sacrificial Sword",       700, CardCost::new(Same, 3, 0), Sword),
            Self::AquilaFavonia         => ("Aquila Favonia",         1000, CardCost::new(Same, 3, 0), Sword),
        }
    }
}

impl super::CardOrd for WeaponCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}