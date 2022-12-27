use crate::{CardCost};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FoodCard {
    MondstadtHashBrown,
    JueyunGuoba,
    AdeptusTemptation,
    LotusFlowerCrisp,
    NorthernSmokedChicken,
    SweetMadame,
    MushroomPizza,
    MintyMeatRolls,
}

impl super::PlayingCard for FoodCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        self.info_dump().1
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn food(&self) -> Option<FoodCard> {
        Some(*self)
    }
}

impl FoodCard {
    fn info_dump(&self) -> (&'static str, Option<Price>, CardCost) {
        match self {
            Self::MondstadtHashBrown    => ("Mondstadt Hash Brown",    None,      CardCost::ONE),
            Self::JueyunGuoba           => ("Jueyun Guoba",            Some(500), CardCost::ZERO),
            Self::AdeptusTemptation     => ("Adeptus' Temptation",     Some(500), CardCost::ANY2),
            Self::LotusFlowerCrisp      => ("Lotus Flower Crisp",      Some(500), CardCost::ONE),
            Self::NorthernSmokedChicken => ("Northern Smoked Chicken", Some(500), CardCost::ZERO),
            Self::SweetMadame           => ("Sweet Madame",            Some(500), CardCost::ZERO),
            Self::MushroomPizza         => ("Mushroom Pizza",          Some(500), CardCost::ONE),
            Self::MintyMeatRolls        => ("Minty Meat Rolls",        Some(500), CardCost::ONE),
        }
    }
}

impl super::CardOrd for FoodCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Food: FoodCard => crate::EventCard => crate::ActionCard => crate::Card);