use crate::{CardCost, DiceCost::{Any, Same}};

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

impl FoodCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn shop_price(&self) -> Option<Price> {
        self.info_dump().1
    }

    pub fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn info_dump(&self) -> (&'static str, Option<Price>, CardCost) {
        match self {
            Self::MondstadtHashBrown    => ("Mondstadt Hash Brown",    None,      CardCost::new(Same, 1, 0)),
            Self::JueyunGuoba           => ("Jueyun Guoba",            Some(500), CardCost::new(Same, 0, 0)),
            Self::AdeptusTemptation     => ("Adeptus' Temptation",     Some(500), CardCost::new(Any,  2, 0)),
            Self::LotusFlowerCrisp      => ("Lotus Flower Crisp",      Some(500), CardCost::new(Same, 1, 0)),
            Self::NorthernSmokedChicken => ("Northern Smoked Chicken", Some(500), CardCost::new(Same, 0, 0)),
            Self::SweetMadame           => ("Sweet Madame",            Some(500), CardCost::new(Same, 0, 0)),
            Self::MushroomPizza         => ("Mushroom Pizza",          Some(500), CardCost::new(Same, 1, 0)),
            Self::MintyMeatRolls        => ("Minty Meat Rolls",        Some(500), CardCost::new(Same, 1, 0)),
        }
    }
}

impl super::CardOrd for FoodCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}