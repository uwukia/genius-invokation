use crate::{CardCost, DiceCost::Exact, Element::*, CharacterCard::{self, *}};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TalentCard {
    KantenSenmyouBlessing,
    SteadyBreathing,
    ShakenNotPurred,
    UndividedHeart,
    ColdBloodedStrike,
    GloriousSeason,
    ProphecyOfSubmersion,
    TheScentRemained,
    FeatherfallJudgment,
    StellarPredator,
    ThunderingPenance,
    Awakening,
    StrategicReserve,
    IGotYourBack,
    FloralSidewinder,
    LandsOfDandelion,
    ChaoticEntropy,
    GrandExpectation,
    FlowingFlame,
    Crossfire,
    NaganoharaMeteorSwarm,
    MirrorCage,
    StreamingSurge,
    StonehideReforged,
    ProliferatingSpores,
    TranscendentAutomaton,
    PaidInFull,
}

impl TalentCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn cost(&self) -> CardCost {
        self.info_dump().1
    }

    /// Returns which character this card is attached to
    /// 
    /// A talent card can only be present in a deck when its attached character is also present
    pub fn character(&self) -> CharacterCard {
        self.info_dump().2
    }

    fn info_dump(&self) -> (&'static str, CardCost, CharacterCard) {
        match self {
            Self::KantenSenmyouBlessing => ("Kanten Senmyou Blessing", CardCost::new(Exact(Cryo),    2, 0), KamisatoAyaka),
            Self::SteadyBreathing =>       ("Steady Breathing",        CardCost::new(Exact(Cryo),    4, 0), Chongyun),
            Self::ShakenNotPurred =>       ("Shaken, Not Purred",      CardCost::new(Exact(Cryo),    4, 0), Diona),
            Self::UndividedHeart =>        ("Undivided Heart",         CardCost::new(Exact(Cryo),    5, 0), Ganyu),
            Self::ColdBloodedStrike =>     ("Cold-Blooded Strike",     CardCost::new(Exact(Cryo),    4, 0), Kaeya),
            Self::GloriousSeason =>        ("Glorious Season",         CardCost::new(Exact(Hydro),   4, 0), Barbara),
            Self::ProphecyOfSubmersion =>  ("Prophecy of Submersion",  CardCost::new(Exact(Hydro),   3, 3), Mona),
            Self::TheScentRemained =>      ("The Scent Remained",      CardCost::new(Exact(Hydro),   4, 0), Xingqiu),
            Self::FeatherfallJudgment =>   ("Featherfall Judgment",    CardCost::new(Exact(Electro), 3, 0), Cyno),
            Self::StellarPredator =>       ("Stellar Predator",        CardCost::new(Exact(Electro), 3, 0), Fischl),
            Self::ThunderingPenance =>     ("Thundering Penance",      CardCost::new(Exact(Electro), 3, 0), Keqing),
            Self::Awakening =>             ("Awakening",               CardCost::new(Exact(Electro), 4, 0), Razor),
            Self::StrategicReserve =>      ("Strategic Reserve",       CardCost::new(Exact(Geo),     4, 0), Ningguang),
            Self::IGotYourBack =>          ("I Got Your Back",         CardCost::new(Exact(Geo),     3, 0), Noelle),
            Self::FloralSidewinder =>      ("Floral Sidewinder",       CardCost::new(Exact(Dendro),  3, 0), Collei),
            Self::LandsOfDandelion =>      ("Lands of Dandelion",      CardCost::new(Exact(Anemo),   4, 3), Jean),
            Self::ChaoticEntropy =>        ("Chaotic Entropy",         CardCost::new(Exact(Anemo),   3, 2), Sucrose),
            Self::GrandExpectation =>      ("Grand Expectation",       CardCost::new(Exact(Pyro),    4, 2), Bennett),
            Self::FlowingFlame =>          ("Flowing Flame",           CardCost::new(Exact(Pyro),    3, 0), Diluc),
            Self::Crossfire =>             ("Crossfire",               CardCost::new(Exact(Pyro),    4, 0), Xiangling),
            Self::NaganoharaMeteorSwarm => ("Naganohara Meteor Swarm", CardCost::new(Exact(Pyro),    2, 0), Yoimiya),
            Self::MirrorCage =>            ("Mirror Cage",             CardCost::new(Exact(Hydro),   4, 0), MirrorMaiden),
            Self::StreamingSurge =>        ("Streaming Surge",         CardCost::new(Exact(Hydro),   4, 3), RhodeiaOfLoch),
            Self::StonehideReforged =>     ("Stonehide Reforged",      CardCost::new(Exact(Geo),     4, 2), StonehideLawachurl),
            Self::ProliferatingSpores =>   ("Proliferating Spores",    CardCost::new(Exact(Dendro),  3, 0), JadeplumeTerrorshroom),
            Self::TranscendentAutomaton => ("Transcendent Automaton",  CardCost::new(Exact(Anemo),   3, 0), MaguuKenki),
            Self::PaidInFull =>            ("Paid in Full",            CardCost::new(Exact(Pyro),    3, 0), FatuiPyroAgent),
        }
    }
}

impl super::CardOrd for TalentCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}