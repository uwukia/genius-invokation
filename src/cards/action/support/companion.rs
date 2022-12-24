use crate::{CardCost, DiceCost::{Same, Any}};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CompanionCard {
    Paimon,
    Katheryne,
    Timaeus,
    Wagner,
    ChefMao,
    Tubby,
    Timmie,
    Liben,
    Ellin,
    LiuSu,
    ChangTheNinth,
    IronTongueTian,
}

impl CompanionCard {
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
            Self::Paimon =>    ("Paimon",    None,      CardCost::new(Same, 3, 0)),
            Self::Katheryne => ("Katheryne", Some(700), CardCost::new(Any,  2, 0)),
            Self::Timaeus =>   ("Timaeus",   Some(700), CardCost::new(Same, 2, 0)),
            Self::Wagner =>    ("Wagner",    Some(700), CardCost::new(Same, 2, 0)),
            Self::ChefMao =>   ("ChefMao",   Some(700), CardCost::new(Same, 1, 0)),
            Self::Tubby =>     ("Tubby",     Some(700), CardCost::new(Same, 2, 0)),
            Self::Timmie =>    ("Timmie",    Some(700), CardCost::new(Same, 0, 0)),
            Self::Liben =>     ("Liben",     Some(700), CardCost::new(Same, 0, 0)),
            Self::Ellin =>     ("Ellin",     Some(700), CardCost::new(Same, 2, 0)),
            Self::LiuSu =>     ("LiuSu",     Some(700), CardCost::new(Same, 1, 0)),
            Self::ChangTheNinth =>  ("ChangTheNinth",  Some(700), CardCost::new(Same, 0, 0)),
            Self::IronTongueTian => ("IronTongueTian", Some(700), CardCost::new(Any,  2, 0)),
        }
    }
}

impl super::CardOrd for CompanionCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}