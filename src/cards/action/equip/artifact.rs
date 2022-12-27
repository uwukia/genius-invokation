use crate::{CardCost, Element::{self, *}};
use super::Price;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArtifactCard {
    AdventurersBandana,
    LuckyDogsSilverCirclet,
    TravelingDoctorsHandkerchief,
    GamblersEarrings,
    InstructorsCap,
    ExilesCirclet,
    BrokenRimesEcho,
    BlizzardStrayer,
    WineStainedTricorne,
    HeartOfDepth,
    WitchsScorchingHat,
    CrimsonWitchOfFlames,
    ThunderSummonersCrown,
    ThunderingFury,
    ViridescentVenerersDiadem,
    ViridescentVenerer,
    MaskOfSolitudeBasalt,
    ArchaicPetra,
    LaurelCoronet,
    DeepwoodMemories,
}

impl super::PlayingCard for ArtifactCard {
    fn name(&self) -> &'static str {
        self.info_dump().0
    }

    fn shop_price(&self) -> Option<Price> {
        Some(self.info_dump().1)
    }

    fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn artifact(&self) -> Option<ArtifactCard> {
        Some(*self)
    }
}

impl ArtifactCard {
    fn info_dump(&self) -> (&'static str, Price, CardCost, Option<Element>) {
        match self {
            Self::AdventurersBandana           => ("Adventurer's Bandana",            500, CardCost::ONE,  None),
            Self::LuckyDogsSilverCirclet       => ("Lucky Dog's Silver Circlet",      500, CardCost::ANY2, None),
            Self::TravelingDoctorsHandkerchief => ("Traveling Doctor's Handkerchief", 500, CardCost::ONE,  None),
            Self::GamblersEarrings             => ("Gambler's Earrings",              500, CardCost::ONE,  None),
            Self::InstructorsCap               => ("Instructor's Cap",                500, CardCost::ANY2, None),
            Self::ExilesCirclet                => ("Exile's Circlet",                 500, CardCost::ANY2, None),
            Self::BrokenRimesEcho              => ("Broken Rime's Echo",              500, CardCost::MATCH2, Some(Cryo)),
            Self::BlizzardStrayer              => ("Blizzard Strayer",                700, CardCost::MATCH3, Some(Cryo)),
            Self::WineStainedTricorne          => ("Wine-Stained Tricorne",           500, CardCost::MATCH2, Some(Hydro)),
            Self::HeartOfDepth                 => ("Heart of Depth",                  700, CardCost::MATCH3, Some(Hydro)),
            Self::WitchsScorchingHat           => ("Witch's Scorching Hat",           500, CardCost::MATCH2, Some(Pyro)),
            Self::CrimsonWitchOfFlames         => ("Crimson Witch of Flames",         700, CardCost::MATCH3, Some(Pyro)),
            Self::ThunderSummonersCrown        => ("Thunder Summoner's Crown",        500, CardCost::MATCH2, Some(Electro)),
            Self::ThunderingFury               => ("Thundering Fury",                 700, CardCost::MATCH3, Some(Electro)),
            Self::ViridescentVenerersDiadem    => ("Viridescent Venerer's Diadem",    500, CardCost::MATCH2, Some(Anemo)),
            Self::ViridescentVenerer           => ("Viridescent Venerer",             700, CardCost::MATCH3, Some(Anemo)),
            Self::MaskOfSolitudeBasalt         => ("Mask of Solitude Basalt",         500, CardCost::MATCH2, Some(Geo)),
            Self::ArchaicPetra                 => ("Archaic Petra",                   700, CardCost::MATCH3, Some(Geo)),
            Self::LaurelCoronet                => ("Laurel Coronet",                  500, CardCost::MATCH2, Some(Dendro)),
            Self::DeepwoodMemories             => ("Deepwood Memories",               700, CardCost::MATCH3, Some(Dendro)),
        }
    }
}

impl super::CardOrd for ArtifactCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}

impl_from!(Artifact: ArtifactCard => crate::EquipmentCard => crate::ActionCard => crate::Card);