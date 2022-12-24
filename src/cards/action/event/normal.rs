use crate::{CardCost, DiceCost::{Same, Any}};

use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum NormalEventCard {
    TossUp,
    SendOff,
    Starsigns,
    CalxsArts,
    QuickKnit,
    Strategize,
    LeaveItToMe,
    GuardiansOath,
    ChangingShifts,
    IHaventLostYet,
    AbyssalSummons,
    MasterOfWeaponry,
    WhenTheCraneReturned,
    TheBestestTravelCompanion,
    BlessingOfTheDivineRelicsInstallation, // i might cut this one's name in half in the future lol
}

impl NormalEventCard {
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
            Self::TossUp =>               ("Toss-Up",                       None,      CardCost::new(Same, 0, 0)),
            Self::SendOff =>              ("Send Off",                      Some(500), CardCost::new(Any,  2, 0)),
            Self::Starsigns =>            ("Starsigns",                     Some(500), CardCost::new(Any,  2, 0)),
            Self::CalxsArts =>            ("Calx's Arts",                   Some(500), CardCost::new(Same, 1, 0)),
            Self::QuickKnit =>            ("Quick Knit",                    Some(500), CardCost::new(Same, 1, 0)),
            Self::Strategize =>           ("Strategize",                    Some(500), CardCost::new(Same, 1, 0)),
            Self::LeaveItToMe =>          ("Leave It To Me!",               Some(500), CardCost::new(Same, 0, 0)),
            Self::GuardiansOath =>        ("Guardian's Oath",               Some(500), CardCost::new(Same, 4, 0)),
            Self::ChangingShifts =>       ("Changing Shifts",               Some(500), CardCost::new(Same, 0, 0)),
            Self::IHaventLostYet =>       ("I haven't Lost Yet!",           Some(500), CardCost::new(Same, 0, 0)),
            Self::AbyssalSummons =>       ("Abyssal Summons",               Some(500), CardCost::new(Same, 2, 0)),
            Self::MasterOfWeaponry =>     ("Master of Weaponry",            Some(500), CardCost::new(Same, 0, 0)),
            Self::WhenTheCraneReturned => ("When The Crane Returned",       Some(500), CardCost::new(Same, 1, 0)),
            Self::TheBestestTravelCompanion => ("The Bestest Travel Companion!", None, CardCost::new(Any,  2, 0)),
            Self::BlessingOfTheDivineRelicsInstallation => {
                ("Blessing of the Divine Relic's Installation", Some(500), CardCost::new(Same, 0, 0))
            },
        }
    }
}

impl super::CardOrd for NormalEventCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}