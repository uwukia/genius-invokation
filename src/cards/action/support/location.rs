use crate::CardCost;

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

impl super::PlayingCard for LocationCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        Some(700)
    }

    fn cost(&self) -> CardCost {
        self.info_dump().1
    }

    fn location(&self) -> Option<LocationCard> {
        Some(*self)
    }
}

impl LocationCard {
    fn info_dump(&self) -> (&'static str, CardCost) {
        match self {
            Self::DawnWinery =>               ("Dawn Winery",                 CardCost::MATCH2),
            Self::FavoniusCathedral =>        ("Favonius Cathedral",          CardCost::MATCH2),
            Self::KnightsOfFavoniusLibrary => ("Knights of Favonius Library", CardCost::ONE),
            Self::JadeChamber =>              ("Jade Chamber",                CardCost::ONE),
            Self::LiyueHarborWharf =>         ("Liyue Harbor Wharf",          CardCost::MATCH2),
            Self::WangshuInn =>               ("Wangshu Inn",                 CardCost::MATCH2),
        }
    }
}

impl super::CardOrd for LocationCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Location: LocationCard => crate::SupportCard => crate::ActionCard => crate::Card);