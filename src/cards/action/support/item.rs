use crate::CardCost;

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ItemCard {
    ParametricTransformer,
    NRE,
}

impl super::PlayingCard for ItemCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        Some(self.info_dump().1)
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn item(&self) -> Option<ItemCard> {
        Some(*self)
    }
}

impl ItemCard {
    fn info_dump(&self) -> (&'static str, Price, CardCost) {
        match self {
            Self::ParametricTransformer => ("Parametric Transformer", 700, CardCost::ANY2),
            Self::NRE =>                   ("NRE",                    700, CardCost::ANY2),
        }
    }
}

impl super::CardOrd for ItemCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Item: ItemCard => crate::SupportCard => crate::ActionCard => crate::Card);