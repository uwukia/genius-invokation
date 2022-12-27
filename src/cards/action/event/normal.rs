use crate::{CardCost, DiceCost::Same};

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

impl super::PlayingCard for NormalEventCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        self.info_dump().1
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn normal_event(&self) -> Option<NormalEventCard> {
        Some(*self)
    }
}

impl NormalEventCard {
    fn info_dump(&self) -> (&'static str, Option<Price>, CardCost) {
        match self {
            Self::TossUp =>               ("Toss-Up",                       None,      CardCost::ZERO),
            Self::SendOff =>              ("Send Off",                      Some(500), CardCost::ANY2),
            Self::Starsigns =>            ("Starsigns",                     Some(500), CardCost::ANY2),
            Self::CalxsArts =>            ("Calx's Arts",                   Some(500), CardCost::ONE),
            Self::QuickKnit =>            ("Quick Knit",                    Some(500), CardCost::ONE),
            Self::Strategize =>           ("Strategize",                    Some(500), CardCost::ONE),
            Self::LeaveItToMe =>          ("Leave It To Me!",               Some(500), CardCost::ZERO),
            Self::GuardiansOath =>        ("Guardian's Oath",               Some(500), CardCost::new(Same, 4, 0)),
            Self::ChangingShifts =>       ("Changing Shifts",               Some(500), CardCost::ZERO),
            Self::IHaventLostYet =>       ("I haven't Lost Yet!",           Some(500), CardCost::ZERO),
            Self::AbyssalSummons =>       ("Abyssal Summons",               Some(500), CardCost::MATCH2),
            Self::MasterOfWeaponry =>     ("Master of Weaponry",            Some(500), CardCost::ZERO),
            Self::WhenTheCraneReturned => ("When The Crane Returned",       Some(500), CardCost::ONE),
            Self::TheBestestTravelCompanion => ("The Bestest Travel Companion!", None, CardCost::ANY2),
            Self::BlessingOfTheDivineRelicsInstallation => {
                ("Blessing of the Divine Relic's Installation", Some(500), CardCost::ZERO)
            },
        }
    }
}

impl super::CardOrd for NormalEventCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Normal: NormalEventCard => crate::EventCard => crate::ActionCard => crate::Card);