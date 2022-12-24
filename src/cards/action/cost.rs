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
    pub(crate) fn new(dice: DiceCost, amount: u8, energy: u8) -> Self {
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

