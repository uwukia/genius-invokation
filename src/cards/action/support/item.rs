use crate::{CardCost, DiceCost::Any};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ItemCard {
    ParametricTransformer,
    NRE,
}

impl ItemCard {
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
            Self::ParametricTransformer => ("Parametric Transformer", 700, CardCost::new(Any, 2, 0)),
            Self::NRE =>                   ("NRE",                    700, CardCost::new(Any, 2, 0)),
        }
    }
}

impl super::CardOrd for ItemCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}