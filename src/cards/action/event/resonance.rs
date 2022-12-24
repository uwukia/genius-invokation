use crate::{CardCost, DiceCost::{Same, Exact}, Element::*};

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
            Self::WovenIce =>          ("Woven Ice",          500, CardCost::new(Same,           0, 0)),
            Self::ShatteringIce =>     ("Shattering Ice",     500, CardCost::new(Exact(Cryo),    1, 0)),
            Self::WovenWaters =>       ("Woven Waters",       500, CardCost::new(Same,           0, 0)),
            Self::SoothingWater =>     ("Soothing Water",     500, CardCost::new(Exact(Hydro),   1, 0)),
            Self::WovenFlames =>       ("Woven Flames",       500, CardCost::new(Same,           0, 0)),
            Self::FerventFlames =>     ("Fervent Flames",     500, CardCost::new(Exact(Pyro),    1, 0)),
            Self::WovenThunder =>      ("Woven Thunder",      500, CardCost::new(Same,           0, 0)),
            Self::HighVoltage =>       ("High Voltage",       500, CardCost::new(Exact(Electro), 1, 0)),
            Self::WovenWinds =>        ("Woven Winds",        500, CardCost::new(Same,           0, 0)),
            Self::ImpetuousWinds =>    ("Impetuous Winds",    500, CardCost::new(Exact(Anemo),   1, 0)),
            Self::WovenStone =>        ("Woven Stone",        500, CardCost::new(Same,           0, 0)),
            Self::EnduringRock =>      ("Enduring Rock",      500, CardCost::new(Exact(Geo),     1, 0)),
            Self::WovenWeeds =>        ("Woven Weeds",        500, CardCost::new(Same,           0, 0)),
            Self::SprawlingGreenery => ("Sprawling Greenery", 500, CardCost::new(Exact(Dendro),  1, 0)),
        }
    }
}

impl super::CardOrd for ElementalResonanceCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}