use crate::Element;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct CardCost {
    dice: DiceCost,
    amount: u8,
    energy: u8,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum DiceCost {
    /// Any dice can be used for this card
    Any,
    /// All dice used must be of the same element
    Same,
    /// The dice used must be of a given specific element
    Exact(Element)
}

impl CardCost {
    /// For cards that cost nothing
    pub(crate) const ZERO: Self = Self { dice: DiceCost::Same, amount: 0, energy: 0 };

    /// For cards that cost one die of any element
    pub(crate) const ONE: Self = Self { dice: DiceCost::Same, amount: 1, energy: 0 };

    /// For cards that cost any two dice of any type
    pub(crate) const ANY2: Self = Self { dice: DiceCost::Any, amount: 2, energy: 0 };

    /// For cards that cost two dice of the same type
    pub(crate) const MATCH2: Self = Self { dice: DiceCost::Same, amount: 2, energy: 0 };

    /// For cards that cost three dice of the same type
    pub(crate) const MATCH3: Self = Self { dice: DiceCost::Same, amount: 3, energy: 0 };

    pub(crate) const fn new(dice: DiceCost, amount: u8, energy: u8) -> Self {
        Self { dice, amount, energy }
    }

    /// Retrieves which type of dice is required to play the given card
    pub fn dice_type(&self) -> DiceCost {
        self.dice
    }

    /// The amount of dice required to play the given card
    pub fn amount(&self) -> u8 {
        self.amount
    }

    /// The amount of character energy required to play this card
    /// 
    /// Most cards don't require energy, which means zero
    pub fn energy(&self) -> u8 {
        self.energy
    }
}

