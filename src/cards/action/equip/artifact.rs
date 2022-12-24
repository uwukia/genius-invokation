use crate::{CardCost, DiceCost::{Any, Same}, Element::{self, *}};
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

impl ArtifactCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn shop_price(&self) -> Price {
        self.info_dump().1
    }

    pub fn cost(&self) -> CardCost {
        self.info_dump().2
    }

    fn info_dump(&self) -> (&'static str, Price, CardCost, Option<Element>) {
        match self {
            Self::AdventurersBandana           => ("Adventurer's Bandana",            500, CardCost::new(Same, 1, 0), None),
            Self::LuckyDogsSilverCirclet       => ("Lucky Dog's Silver Circlet",      500, CardCost::new(Any,  2, 0), None),
            Self::TravelingDoctorsHandkerchief => ("Traveling Doctor's Handkerchief", 500, CardCost::new(Same, 1, 0), None),
            Self::GamblersEarrings             => ("Gambler's Earrings",              500, CardCost::new(Same, 1, 0), None),
            Self::InstructorsCap               => ("Instructor's Cap",                500, CardCost::new(Any,  2, 0), None),
            Self::ExilesCirclet                => ("Exile's Circlet",                 500, CardCost::new(Any,  2, 0), None),
            Self::BrokenRimesEcho              => ("Broken Rime's Echo",              500, CardCost::new(Same, 2, 0), Some(Cryo)),
            Self::BlizzardStrayer              => ("Blizzard Strayer",                700, CardCost::new(Same, 3, 0), Some(Cryo)),
            Self::WineStainedTricorne          => ("Wine-Stained Tricorne",           500, CardCost::new(Same, 2, 0), Some(Hydro)),
            Self::HeartOfDepth                 => ("Heart of Depth",                  700, CardCost::new(Same, 3, 0), Some(Hydro)),
            Self::WitchsScorchingHat           => ("Witch's Scorching Hat",           500, CardCost::new(Same, 2, 0), Some(Pyro)),
            Self::CrimsonWitchOfFlames         => ("Crimson Witch of Flames",         700, CardCost::new(Same, 3, 0), Some(Pyro)),
            Self::ThunderSummonersCrown        => ("Thunder Summoner's Crown",        500, CardCost::new(Same, 2, 0), Some(Electro)),
            Self::ThunderingFury               => ("Thundering Fury",                 700, CardCost::new(Same, 3, 0), Some(Electro)),
            Self::ViridescentVenerersDiadem    => ("Viridescent Venerer's Diadem",    500, CardCost::new(Same, 2, 0), Some(Anemo)),
            Self::ViridescentVenerer           => ("Viridescent Venerer",             700, CardCost::new(Same, 3, 0), Some(Anemo)),
            Self::MaskOfSolitudeBasalt         => ("Mask of Solitude Basalt",         500, CardCost::new(Same, 2, 0), Some(Geo)),
            Self::ArchaicPetra                 => ("Archaic Petra",                   700, CardCost::new(Same, 3, 0), Some(Geo)),
            Self::LaurelCoronet                => ("Laurel Coronet",                  500, CardCost::new(Same, 2, 0), Some(Dendro)),
            Self::DeepwoodMemories             => ("Deepwood Memories",               700, CardCost::new(Same, 3, 0), Some(Dendro)),
        }
    }
}

impl super::CardOrd for ArtifactCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}