use crate::Element::{self, *};

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

    pub fn weapon(&self) -> WeaponType {
        self.info_dump().2
    }

    pub fn faction(&self) -> Faction {
        self.info_dump().3
    }

    fn info_dump(&self) -> (&'static str, Element, WeaponType, Faction) {
        match self {
            Self::KamisatoAyaka => ("Kamisato Ayaka", Cryo,    Sword,    Inazuma),
            Self::Chongyun =>      ("Chongyun",       Cryo,    Claymore, Liyue),
            Self::Diona =>         ("Diona",          Cryo,    Bow,      Mondstadt),
            Self::Ganyu =>         ("Ganyu",          Cryo,    Bow,      Liyue),
            Self::Kaeya =>         ("Kaeya",          Cryo,    Sword,    Mondstadt),
            Self::Barbara =>       ("Barbara",        Hydro,   Catalyst, Mondstadt),
            Self::Mona =>          ("Mona",           Hydro,   Catalyst, Mondstadt),
            Self::Xingqiu =>       ("Xingqiu",        Hydro,   Sword,    Liyue),
            Self::Cyno =>          ("Cyno",           Electro, Polearm,  Sumeru),
            Self::Fischl =>        ("Fischl",         Electro, Bow,      Mondstadt),
            Self::Keqing =>        ("Keqing",         Electro, Sword,    Liyue),
            Self::Razor =>         ("Razor",          Electro, Claymore, Mondstadt),
            Self::Ningguang =>     ("Ningguang",      Geo,     Catalyst, Liyue),
            Self::Noelle =>        ("Noelle",         Geo,     Claymore, Mondstadt),
            Self::Collei =>        ("Collei",         Dendro,  Bow,      Sumeru),
            Self::Jean =>          ("Jean",           Anemo,   Sword,    Mondstadt),
            Self::Sucrose =>       ("Sucrose",        Anemo,   Catalyst, Mondstadt),
            Self::Bennett =>       ("Bennett",        Pyro,    Sword,    Mondstadt),
            Self::Diluc =>         ("Diluc",          Pyro,    Claymore, Mondstadt),
            Self::Xiangling =>     ("Xiangling",      Pyro,    Polearm,  Liyue),
            Self::Yoimiya =>       ("Yoimiya",        Pyro,    Bow,      Inazuma),
            Self::MirrorMaiden =>          ("Mirror Maiden",          Hydro,  OtherWeapons, Fatui),
            Self::RhodeiaOfLoch =>         ("Rhodeia of Loch",        Hydro,  OtherWeapons, Monster),
            Self::StonehideLawachurl =>    ("Stonehide Lawachurl",    Geo,    None,         Monster),
            Self::JadeplumeTerrorshroom => ("Jadeplume Terrorshroom", Dendro, None,         Monster),
            Self::MaguuKenki =>            ("Maguu Kenki",            Anemo,  OtherWeapons, Monster),
            Self::FatuiPyroAgent =>        ("Fatui Pyro Agent",       Pyro,   OtherWeapons, Fatui),
        }
    }
}