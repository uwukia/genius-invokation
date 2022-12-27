use crate::CardCost;

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

impl super::PlayingCard for CompanionCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        self.info_dump().1
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn companion(&self) -> Option<CompanionCard> {
        Some(*self)
    }
}

impl CompanionCard {
    fn info_dump(&self) -> (&'static str, Option<Price>, CardCost) {
        match self {
            Self::Paimon =>    ("Paimon",    None,      CardCost::MATCH3),
            Self::Katheryne => ("Katheryne", Some(700), CardCost::ANY2),
            Self::Timaeus =>   ("Timaeus",   Some(700), CardCost::MATCH2),
            Self::Wagner =>    ("Wagner",    Some(700), CardCost::MATCH2),
            Self::ChefMao =>   ("Chef Mao",  Some(700), CardCost::ONE),
            Self::Tubby =>     ("Tubby",     Some(700), CardCost::MATCH2),
            Self::Timmie =>    ("Timmie",    Some(700), CardCost::ZERO),
            Self::Liben =>     ("Liben",     Some(700), CardCost::ZERO),
            Self::Ellin =>     ("Ellin",     Some(700), CardCost::MATCH2),
            Self::LiuSu =>     ("Liu Su",    Some(700), CardCost::ONE),
            Self::ChangTheNinth =>  ("Chang the Ninth",  Some(700), CardCost::ZERO),
            Self::IronTongueTian => ("Iron Tongue Tian", Some(700), CardCost::ANY2),
        }
    }
}

impl super::CardOrd for CompanionCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Companion: CompanionCard => crate::SupportCard => crate::ActionCard => crate::Card);