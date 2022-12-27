use crate::{CardCost, DiceCost::Exact, Element::{self, *}};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ElementalResonanceCard {
    WovenIce,
    ShatteringIce,
    WovenWaters,
    SoothingWater,
    WovenFlames,
    FerventFlames,
    WovenThunder,
    HighVoltage,
    WovenWinds,
    ImpetuousWinds,
    WovenStone,
    EnduringRock,
    WovenWeeds,
    SprawlingGreenery,
}

impl ElementalResonanceCard {
    /// Returns which element this resonance card is attached to
    pub fn element(&self) -> Element {
        self.info_dump().1
    }
}

impl super::PlayingCard for ElementalResonanceCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        Some(500)
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn resonance(&self) -> Option<ElementalResonanceCard> {
        Some(*self)
    }
}

impl ElementalResonanceCard {
    fn info_dump(&self) -> (&'static str, Element, CardCost) {
        match self {
            Self::WovenIce =>          ("Woven Ice",          Cryo,    CardCost::ZERO),
            Self::ShatteringIce =>     ("Shattering Ice",     Cryo,    CardCost::new(Exact(Cryo),    1, 0)),
            Self::WovenWaters =>       ("Woven Waters",       Hydro,   CardCost::ZERO),
            Self::SoothingWater =>     ("Soothing Water",     Hydro,   CardCost::new(Exact(Hydro),   1, 0)),
            Self::WovenFlames =>       ("Woven Flames",       Pyro,    CardCost::ZERO),
            Self::FerventFlames =>     ("Fervent Flames",     Pyro,    CardCost::new(Exact(Pyro),    1, 0)),
            Self::WovenThunder =>      ("Woven Thunder",      Electro, CardCost::ZERO),
            Self::HighVoltage =>       ("High Voltage",       Electro, CardCost::new(Exact(Electro), 1, 0)),
            Self::WovenWinds =>        ("Woven Winds",        Anemo,   CardCost::ZERO),
            Self::ImpetuousWinds =>    ("Impetuous Winds",    Anemo,   CardCost::new(Exact(Anemo),   1, 0)),
            Self::WovenStone =>        ("Woven Stone",        Geo,     CardCost::ZERO),
            Self::EnduringRock =>      ("Enduring Rock",      Geo,     CardCost::new(Exact(Geo),     1, 0)),
            Self::WovenWeeds =>        ("Woven Weeds",        Dendro,  CardCost::ZERO),
            Self::SprawlingGreenery => ("Sprawling Greenery", Dendro,  CardCost::new(Exact(Dendro),  1, 0)),
        }
    }
}

impl super::CardOrd for ElementalResonanceCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Resonance: ElementalResonanceCard => crate::EventCard => crate::ActionCard => crate::Card);