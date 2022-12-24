/// # Retrieving a Deck from a URL
/// 
/// Example:
/// 
/// https://genshin.hotgames.gg/tcg/deck-builder?deck=1.6.MC.MD.MF.MG.MH.MI.MJ.MK.ML.MM.MN.MO.MP.MV.MY.e.g8.gB.gD.gF.gb.ge.gh.gk.gt.gv.gx.gz.wj.wl.wm&ver=1&lang=en&author=DefaultDeck

use std::{fmt, iter};
use crate::*;

const URL_STARTER: &'static str = "genshin.hotgames.gg/tcg/deck-builder?deck=";

#[derive(Debug)]
pub enum UrlDeckError<'s> {
    InvalidUrl(&'s str),
    UnknownVersion(&'s str),
    UnknownCard(&'s str),
}

pub fn deck_from_url<'s>(url: &'s str) -> Result<impl Iterator<Item=Card> + 's, UrlDeckError<'s>> {
    let index = url.find(URL_STARTER).ok_or_else(|| UrlDeckError::InvalidUrl(url))?;
    let start = index + URL_STARTER.len();

    let mut split = url[start..].split('&');
    let deck  = split.next().ok_or_else(|| UrlDeckError::InvalidUrl(url))?;
    let version = get_version(split.next()).ok_or_else(|| UrlDeckError::InvalidUrl(url))?;

    if version == "1" {
        decode(deck)
    } else {
        Err(UrlDeckError::UnknownVersion(version))
    }
}

fn decode<'s>(deck: &'s str) -> Result<impl Iterator<Item=Card> + 's, UrlDeckError<'s>> {
    verify_iterator(deck.split('.').map(|card| decode_card(card)))?;

    Ok(deck.split('.').map(|card| decode_card(card).unwrap()).flatten())
}

fn decode_card<'s>(card: &'s str) -> Result<impl Iterator<Item=Card> + fmt::Debug + 's, &'s str> {
    let (card, raw_amount) = card.split_once('-').unwrap_or((card, "1"));
    let amount = str::parse::<usize>(raw_amount).map_err(|_| card)?;

    let card = match card.len() {
        1 => decode_character_card(card),
        2 => decode_action_card(card),
        _ => Err(card)
    }?;

    Ok(iter::repeat(card).take(amount))
}

fn get_version<'s>(version: Option<&'s str>) -> Option<&'s str> {
    version.map(|str| str.strip_prefix("ver=")).flatten()
}

fn verify_iterator<'s, I>(
    mut iterator: impl Iterator<Item=Result<I, &'s str>> + 's
) -> Result<(), UrlDeckError<'s>>
    where I: Iterator<Item=Card> + fmt::Debug
{
    if let Some(error) = iterator.find(|item| item.is_err()) {
        Err(UrlDeckError::UnknownCard(error.unwrap_err()))
    } else {
        Ok(())
    }
}

fn decode_character_card<'s>(card: &'s str) -> Result<Card, &'s str> {
    match card {
        "0" => Ok(Card::Character(CharacterCard::Ganyu)),
        "1" => Ok(Card::Character(CharacterCard::Kaeya)),
        "2" => Ok(Card::Character(CharacterCard::Chongyun)),
        "3" => Ok(Card::Character(CharacterCard::KamisatoAyaka)),
        "4" => Ok(Card::Character(CharacterCard::Xingqiu)),
        "5" => Ok(Card::Character(CharacterCard::Mona)),
        "6" => Ok(Card::Character(CharacterCard::Diluc)),
        "7" => Ok(Card::Character(CharacterCard::Xiangling)),
        "8" => Ok(Card::Character(CharacterCard::Bennett)),
        "a" => Ok(Card::Character(CharacterCard::Yoimiya)),
        "b" => Ok(Card::Character(CharacterCard::Fischl)),
        "c" => Ok(Card::Character(CharacterCard::Razor)),
        "d" => Ok(Card::Character(CharacterCard::Keqing)),
        "e" => Ok(Card::Character(CharacterCard::Sucrose)),
        "f" => Ok(Card::Character(CharacterCard::Jean)),
        "g" => Ok(Card::Character(CharacterCard::Ningguang)),
        "h" => Ok(Card::Character(CharacterCard::Noelle)),
        "i" => Ok(Card::Character(CharacterCard::Collei)),
        "j" => Ok(Card::Character(CharacterCard::RhodeiaOfLoch)),
        "k" => Ok(Card::Character(CharacterCard::FatuiPyroAgent)),
        "l" => Ok(Card::Character(CharacterCard::MaguuKenki)),
        "m" => Ok(Card::Character(CharacterCard::StonehideLawachurl)),
        "n" => Ok(Card::Character(CharacterCard::Diona)),
        "o" => Ok(Card::Character(CharacterCard::Cyno)),
        "p" => Ok(Card::Character(CharacterCard::Barbara)),
        "q" => Ok(Card::Character(CharacterCard::MirrorMaiden)),
        "r" => Ok(Card::Character(CharacterCard::JadeplumeTerrorshroom)),
        _ => Err(card)
    }
}

fn decode_action_card<'s>(card: &'s str) -> Result<Card, &'s str> {
    if let Some(decoded) = decode_equip_card(card) { return Ok(Card::Action(decoded)) }
    if let Some(decoded) = decode_event_card(card) { return Ok(Card::Action(decoded)) }
    if let Some(decoded) = decode_support_card(card) { return Ok(Card::Action(decoded)) }

    Err(card)
}

fn decode_equip_card(card: &str) -> Option<ActionCard> {
    match card {
        "84" => Some(EquipmentCard::Talent(TalentCard::UndividedHeart)),
        "85" => Some(EquipmentCard::Talent(TalentCard::ColdBloodedStrike)),
        "86" => Some(EquipmentCard::Talent(TalentCard::SteadyBreathing)),
        "87" => Some(EquipmentCard::Talent(TalentCard::KantenSenmyouBlessing)),
        "88" => Some(EquipmentCard::Talent(TalentCard::TheScentRemained)),
        "89" => Some(EquipmentCard::Talent(TalentCard::ProphecyOfSubmersion)),
        "8a" => Some(EquipmentCard::Talent(TalentCard::FlowingFlame)),
        "8b" => Some(EquipmentCard::Talent(TalentCard::Crossfire)),
        "8c" => Some(EquipmentCard::Talent(TalentCard::GrandExpectation)),
        "8e" => Some(EquipmentCard::Talent(TalentCard::NaganoharaMeteorSwarm)),
        "8f" => Some(EquipmentCard::Talent(TalentCard::StellarPredator)),
        "8g" => Some(EquipmentCard::Talent(TalentCard::Awakening)),
        "8h" => Some(EquipmentCard::Talent(TalentCard::ThunderingPenance)),
        "8i" => Some(EquipmentCard::Talent(TalentCard::ChaoticEntropy)),
        "8j" => Some(EquipmentCard::Talent(TalentCard::LandsOfDandelion)),
        "8k" => Some(EquipmentCard::Talent(TalentCard::StrategicReserve)),
        "8l" => Some(EquipmentCard::Talent(TalentCard::IGotYourBack)),
        "8m" => Some(EquipmentCard::Talent(TalentCard::FloralSidewinder)),
        "8n" => Some(EquipmentCard::Talent(TalentCard::StreamingSurge)),
        "8o" => Some(EquipmentCard::Talent(TalentCard::PaidInFull)),
        "8p" => Some(EquipmentCard::Talent(TalentCard::TranscendentAutomaton)),
        "8q" => Some(EquipmentCard::Talent(TalentCard::StonehideReforged)),
        "8r" => Some(EquipmentCard::Talent(TalentCard::ShakenNotPurred)),
        "8s" => Some(EquipmentCard::Talent(TalentCard::FeatherfallJudgment)),
        "8t" => Some(EquipmentCard::Talent(TalentCard::GloriousSeason)),
        "8u" => Some(EquipmentCard::Talent(TalentCard::MirrorCage)),
        "8v" => Some(EquipmentCard::Talent(TalentCard::ProliferatingSpores)),
        "g8" => Some(EquipmentCard::Weapon(WeaponCard::MagicGuide)),
        "g9" => Some(EquipmentCard::Weapon(WeaponCard::SacrificialFragments)),
        "ga" => Some(EquipmentCard::Weapon(WeaponCard::SkywardAtlas)),
        "gb" => Some(EquipmentCard::Weapon(WeaponCard::RavenBow)),
        "gc" => Some(EquipmentCard::Weapon(WeaponCard::SacrificialBow)),
        "gd" => Some(EquipmentCard::Weapon(WeaponCard::SkywardHarp)),
        "ge" => Some(EquipmentCard::Weapon(WeaponCard::WhiteIronGreatsword)),
        "gf" => Some(EquipmentCard::Weapon(WeaponCard::SacrificialGreatsword)),
        "gg" => Some(EquipmentCard::Weapon(WeaponCard::WolfsGravestone)),
        "gh" => Some(EquipmentCard::Weapon(WeaponCard::WhiteTassel)),
        "gi" => Some(EquipmentCard::Weapon(WeaponCard::LithicSpear)),
        "gj" => Some(EquipmentCard::Weapon(WeaponCard::SkywardSpine)),
        "gk" => Some(EquipmentCard::Weapon(WeaponCard::TravelersHandySword)),
        "gl" => Some(EquipmentCard::Weapon(WeaponCard::SacrificialSword)),
        "gm" => Some(EquipmentCard::Weapon(WeaponCard::AquilaFavonia)),
        "gn" => Some(EquipmentCard::Artifact(ArtifactCard::AdventurersBandana)),
        "go" => Some(EquipmentCard::Artifact(ArtifactCard::LuckyDogsSilverCirclet)),
        "gp" => Some(EquipmentCard::Artifact(ArtifactCard::TravelingDoctorsHandkerchief)),
        "gq" => Some(EquipmentCard::Artifact(ArtifactCard::GamblersEarrings)),
        "gr" => Some(EquipmentCard::Artifact(ArtifactCard::InstructorsCap)),
        "gs" => Some(EquipmentCard::Artifact(ArtifactCard::ExilesCirclet)),
        "gt" => Some(EquipmentCard::Artifact(ArtifactCard::BrokenRimesEcho)),
        "gu" => Some(EquipmentCard::Artifact(ArtifactCard::BlizzardStrayer)),
        "gv" => Some(EquipmentCard::Artifact(ArtifactCard::WineStainedTricorne)),
        "gw" => Some(EquipmentCard::Artifact(ArtifactCard::HeartOfDepth)),
        "gx" => Some(EquipmentCard::Artifact(ArtifactCard::WitchsScorchingHat)),
        "gy" => Some(EquipmentCard::Artifact(ArtifactCard::CrimsonWitchOfFlames)),
        "gz" => Some(EquipmentCard::Artifact(ArtifactCard::ThunderSummonersCrown)),
        "gA" => Some(EquipmentCard::Artifact(ArtifactCard::ThunderingFury)),
        "gB" => Some(EquipmentCard::Artifact(ArtifactCard::ViridescentVenerersDiadem)),
        "gC" => Some(EquipmentCard::Artifact(ArtifactCard::ViridescentVenerer)),
        "gD" => Some(EquipmentCard::Artifact(ArtifactCard::MaskOfSolitudeBasalt)),
        "gE" => Some(EquipmentCard::Artifact(ArtifactCard::ArchaicPetra)),
        "gF" => Some(EquipmentCard::Artifact(ArtifactCard::LaurelCoronet)),
        "gG" => Some(EquipmentCard::Artifact(ArtifactCard::DeepwoodMemories)),
        _ => None
    }.map(|card| ActionCard::Equipment(card))
}

fn decode_event_card(card: &str) -> Option<ActionCard> {
    match card {
        "Mo" => Some(EventCard::Resonance(ElementalResonanceCard::WovenIce)),
        "Mp" => Some(EventCard::Resonance(ElementalResonanceCard::ShatteringIce)),
        "Mq" => Some(EventCard::Resonance(ElementalResonanceCard::WovenWaters)),
        "Mr" => Some(EventCard::Resonance(ElementalResonanceCard::SoothingWater)),
        "Ms" => Some(EventCard::Resonance(ElementalResonanceCard::WovenFlames)),
        "Mt" => Some(EventCard::Resonance(ElementalResonanceCard::FerventFlames)),
        "Mu" => Some(EventCard::Resonance(ElementalResonanceCard::WovenThunder)),
        "Mv" => Some(EventCard::Resonance(ElementalResonanceCard::HighVoltage)),
        "Mw" => Some(EventCard::Resonance(ElementalResonanceCard::WovenWinds)),
        "Mx" => Some(EventCard::Resonance(ElementalResonanceCard::ImpetuousWinds)),
        "My" => Some(EventCard::Resonance(ElementalResonanceCard::WovenStone)),
        "Mz" => Some(EventCard::Resonance(ElementalResonanceCard::EnduringRock)),
        "MA" => Some(EventCard::Resonance(ElementalResonanceCard::WovenWeeds)),
        "MB" => Some(EventCard::Resonance(ElementalResonanceCard::SprawlingGreenery)),
        "MC" => Some(EventCard::Normal(NormalEventCard::TheBestestTravelCompanion)),
        "MD" => Some(EventCard::Normal(NormalEventCard::ChangingShifts)),
        "ME" => Some(EventCard::Normal(NormalEventCard::TossUp)),
        "MF" => Some(EventCard::Normal(NormalEventCard::Strategize)),
        "MG" => Some(EventCard::Normal(NormalEventCard::IHaventLostYet)),
        "MH" => Some(EventCard::Normal(NormalEventCard::LeaveItToMe)),
        "MI" => Some(EventCard::Normal(NormalEventCard::WhenTheCraneReturned)),
        "MJ" => Some(EventCard::Normal(NormalEventCard::Starsigns)),
        "MK" => Some(EventCard::Normal(NormalEventCard::CalxsArts)),
        "ML" => Some(EventCard::Normal(NormalEventCard::MasterOfWeaponry)),
        "MM" => Some(EventCard::Normal(NormalEventCard::BlessingOfTheDivineRelicsInstallation)),
        "MN" => Some(EventCard::Normal(NormalEventCard::QuickKnit)),
        "MO" => Some(EventCard::Normal(NormalEventCard::SendOff)),
        "MP" => Some(EventCard::Normal(NormalEventCard::GuardiansOath)),
        "MQ" => Some(EventCard::Normal(NormalEventCard::AbyssalSummons)),
        "MR" => Some(EventCard::Food(FoodCard::JueyunGuoba)),
        "MS" => Some(EventCard::Food(FoodCard::AdeptusTemptation)),
        "MT" => Some(EventCard::Food(FoodCard::LotusFlowerCrisp)),
        "MU" => Some(EventCard::Food(FoodCard::NorthernSmokedChicken)),
        "MV" => Some(EventCard::Food(FoodCard::SweetMadame)),
        "MW" => Some(EventCard::Food(FoodCard::MondstadtHashBrown)),
        "MX" => Some(EventCard::Food(FoodCard::MushroomPizza)),
        "MY" => Some(EventCard::Food(FoodCard::MintyMeatRolls)),
        _ => None
    }.map(|card| ActionCard::Event(card))
}

fn decode_support_card(card: &str) -> Option<ActionCard> {
    match card {
        "wg" => Some(SupportCard::Location(LocationCard::LiyueHarborWharf)),
        "wh" => Some(SupportCard::Location(LocationCard::KnightsOfFavoniusLibrary)),
        "wi" => Some(SupportCard::Location(LocationCard::JadeChamber)),
        "wj" => Some(SupportCard::Location(LocationCard::DawnWinery)),
        "wk" => Some(SupportCard::Location(LocationCard::WangshuInn)),
        "wl" => Some(SupportCard::Location(LocationCard::FavoniusCathedral)),
        "wm" => Some(SupportCard::Companion(CompanionCard::Paimon)),
        "wn" => Some(SupportCard::Companion(CompanionCard::Katheryne)),
        "wo" => Some(SupportCard::Companion(CompanionCard::Timaeus)),
        "wp" => Some(SupportCard::Companion(CompanionCard::Wagner)),
        "wq" => Some(SupportCard::Companion(CompanionCard::ChefMao)),
        "wr" => Some(SupportCard::Companion(CompanionCard::Tubby)),
        "ws" => Some(SupportCard::Companion(CompanionCard::Timmie)),
        "wt" => Some(SupportCard::Companion(CompanionCard::Liben)),
        "wu" => Some(SupportCard::Companion(CompanionCard::ChangTheNinth)),
        "wv" => Some(SupportCard::Companion(CompanionCard::Ellin)),
        "ww" => Some(SupportCard::Companion(CompanionCard::IronTongueTian)),
        "wx" => Some(SupportCard::Companion(CompanionCard::LiuSu)),
        "wy" => Some(SupportCard::Item(ItemCard::ParametricTransformer)),
        "wz" => Some(SupportCard::Item(ItemCard::NRE)),
        _ => None
    }.map(|card| ActionCard::Support(card))
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_deck() {
        let url = "https://genshin.hotgames.gg/tcg/deck-builder?deck=1.6.MC.MD.MF.MG.MH.MI.MJ.MK.ML.MM.MN.MO.MP.MV.MY.e.g8.gB.gD.gF.gb.ge.gh.gk.gt.gv.gx.gz.wj.wl.wm&ver=1&lang=en&author=DefaultDeck";

        assert!(super::deck_from_url(url).is_ok());
    }
}