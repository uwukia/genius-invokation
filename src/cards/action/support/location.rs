use crate::{CardCost, DiceCost::Same};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocationCard {
    DawnWinery,
    FavoniusCathedral,
    KnightsOfFavoniusLibrary,
    JadeChamber,
    LiyueHarborWharf,
    WangshuInn,
}

impl LocationCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn shop_price(&self) -> Price {
        self.info_dump().1
    }

    pub fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn info_dump(&self) -> (&'static str, Price, CardCost) {
        match self {
            Self::DawnWinery =>               ("Dawn Winery",                 700, CardCost::new(Same, 2, 0)),
            Self::FavoniusCathedral =>        ("Favonius Cathedral",          700, CardCost::new(Same, 2, 0)),
            Self::KnightsOfFavoniusLibrary => ("Knights of Favonius Library", 700, CardCost::new(Same, 1, 0)),
            Self::JadeChamber =>              ("Jade Chamber",                700, CardCost::new(Same, 1, 0)),
            Self::LiyueHarborWharf =>         ("Liyue Harbor Wharf",          700, CardCost::new(Same, 2, 0)),
            Self::WangshuInn =>               ("Wangshu Inn",                 700, CardCost::new(Same, 2, 0)),
        }
    }
}

impl super::CardOrd for LocationCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}