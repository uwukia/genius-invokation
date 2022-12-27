use crate::{Card, Element::{self, *}};

pub use weapon::WeaponType;
mod weapon;

pub use faction::Faction;
mod faction;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CharacterCard {
    KamisatoAyaka,
    Chongyun,
    Diona,
    Ganyu,
    Kaeya,
    Barbara,
    Mona,
    Xingqiu,
    Cyno,
    Fischl,
    Keqing,
    Razor,
    Ningguang,
    Noelle,
    Collei,
    Jean,
    Sucrose,
    Bennett,
    Diluc,
    Xiangling,
    Yoimiya,
    MirrorMaiden,
    RhodeiaOfLoch,
    StonehideLawachurl,
    JadeplumeTerrorshroom,
    MaguuKenki,
    FatuiPyroAgent,
}

use WeaponType::*;
use Faction::*;

impl CharacterCard {
    pub fn name(&self) -> &'static str {
        self.info_dump().0
    }

    pub fn element(&self) -> Element {
        self.info_dump().1
    }

    /// The weapon type this character can hold
    /// 
    /// Returns `None` for characters listed as "Other Weapon"
    pub fn weapon(&self) -> Option<WeaponType> {
        self.info_dump().2
    }

    pub fn faction(&self) -> Faction {
        self.info_dump().3  
    }

    fn info_dump(&self) -> (&'static str, Element, Option<WeaponType>, Faction) {
        match self {
            Self::KamisatoAyaka => ("Kamisato Ayaka", Cryo,    Some(Sword),    Inazuma),
            Self::Chongyun =>      ("Chongyun",       Cryo,    Some(Claymore), Liyue),
            Self::Diona =>         ("Diona",          Cryo,    Some(Bow),      Mondstadt),
            Self::Ganyu =>         ("Ganyu",          Cryo,    Some(Bow),      Liyue),
            Self::Kaeya =>         ("Kaeya",          Cryo,    Some(Sword),    Mondstadt),
            Self::Barbara =>       ("Barbara",        Hydro,   Some(Catalyst), Mondstadt),
            Self::Mona =>          ("Mona",           Hydro,   Some(Catalyst), Mondstadt),
            Self::Xingqiu =>       ("Xingqiu",        Hydro,   Some(Sword),    Liyue),
            Self::Cyno =>          ("Cyno",           Electro, Some(Polearm),  Sumeru),
            Self::Fischl =>        ("Fischl",         Electro, Some(Bow),      Mondstadt),
            Self::Keqing =>        ("Keqing",         Electro, Some(Sword),    Liyue),
            Self::Razor =>         ("Razor",          Electro, Some(Claymore), Mondstadt),
            Self::Ningguang =>     ("Ningguang",      Geo,     Some(Catalyst), Liyue),
            Self::Noelle =>        ("Noelle",         Geo,     Some(Claymore), Mondstadt),
            Self::Collei =>        ("Collei",         Dendro,  Some(Bow),      Sumeru),
            Self::Jean =>          ("Jean",           Anemo,   Some(Sword),    Mondstadt),
            Self::Sucrose =>       ("Sucrose",        Anemo,   Some(Catalyst), Mondstadt),
            Self::Bennett =>       ("Bennett",        Pyro,    Some(Sword),    Mondstadt),
            Self::Diluc =>         ("Diluc",          Pyro,    Some(Claymore), Mondstadt),
            Self::Xiangling =>     ("Xiangling",      Pyro,    Some(Polearm),  Liyue),
            Self::Yoimiya =>       ("Yoimiya",        Pyro,    Some(Bow),      Inazuma),
            Self::MirrorMaiden =>          ("Mirror Maiden",          Hydro,  None, Fatui),
            Self::RhodeiaOfLoch =>         ("Rhodeia of Loch",        Hydro,  None, Monster),
            Self::StonehideLawachurl =>    ("Stonehide Lawachurl",    Geo,    None, Monster),
            Self::JadeplumeTerrorshroom => ("Jadeplume Terrorshroom", Dendro, None, Monster),
            Self::MaguuKenki =>            ("Maguu Kenki",            Anemo,  None, Monster),
            Self::FatuiPyroAgent =>        ("Fatui Pyro Agent",       Pyro,   None, Fatui),
        }
    }
}

impl From<CharacterCard> for Card {
    fn from(card: CharacterCard) -> Card {
        Card::Character(card)
    }
}

impl super::CardOrd for CharacterCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u32).cmp(&(*other as u32))
    }
}