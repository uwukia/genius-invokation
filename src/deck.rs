use crate::{
    PlayingCard, CharacterCard, ActionCard, Card, Element,
    EquipmentCard, TalentCard, 
    EventCard, ElementalResonanceCard,
};
use super::cards::CardOrd;

/// A deck for Genius Invokation TCG
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    characters: [CharacterCard; 3],
    actions: [ActionCard; 30],
}

impl Deck {
    pub fn has_character(&self, card: CharacterCard) -> bool {
        let x = self.characters;

        card == x[0] || card == x[1] || card == x[2]
    }

    pub fn contains(&self, card: ActionCard) -> bool {
        for i in 0..30 {
            if card == self.actions[i] { return true }
        }

        false
    }

    pub fn iter(&self) -> IterAction<'_> {
        IterAction { array: &self.actions, index: 0, unique: false }
    }

    pub fn iter_unique(&self) -> IterAction<'_> {
        IterAction { array: &self.actions, index: 0, unique: true }
    }

    pub fn from_iter(iter: impl IntoIterator<Item=Card>) -> Result<Self, DeckError> {
        let mut cards = iter.into_iter();

        let mut char_vec   = Vec::with_capacity(3);
        let mut action_vec = Vec::with_capacity(30);

        while let Some(card) = cards.next() {
            match card {
                Card::Character(character) => {
                    if char_vec.len() == 3 {
                        return Err(DeckError::TooManyCharacterCards)
                    } else {
                        char_vec.push(character)
                    }
                },
                Card::Action(action) => {
                    if action_vec.len() == 30 {
                        return Err(DeckError::TooManyActionCards)
                    } else {
                        action_vec.push(action)
                    }
                }
            }
        }

        if char_vec.len() != 3 { return   Err(DeckError::NotEnoughCharacterCards(char_vec.len() as u8)) }
        if action_vec.len() != 30 { return Err(DeckError::NotEnoughActionCards(action_vec.len() as u8)) }

        char_vec.sort_by(|a, b| a.cmp(b));
        action_vec.sort_by(|a, b| a.cmp(b));

        let characters = <[CharacterCard; 3]>::try_from(char_vec).unwrap();
        let actions    = <[ActionCard; 30]>::try_from(action_vec).unwrap();

        Deck::verify(&characters, &actions)?;

        Ok(Self { characters, actions })
    }

    pub fn from_exact(mut characters: [CharacterCard; 3], mut actions: [ActionCard; 30]) -> Result<Self, DeckError> {
        characters.sort_by(|a, b| a.cmp(b));
        actions.sort_by(|a, b| a.cmp(b));

        Deck::verify(&characters, &actions)?;

        Ok(Self { characters, actions })
    }

    fn verify(characters: &[CharacterCard; 3], actions: &[ActionCard; 30]) -> Result<(), DeckError> {
        if characters[0] == characters[1] || characters[1] == characters[2] {
            // we don't have to verify if characters[0] == characters[2] because the array
            // is assumed to be sorted already
            return Err(DeckError::CharacterAppearsMoreThanOnce(characters[1]))
        }

        for i in 0..28 {
            // similarly, since actions is also sorted, we can check for triplicates by just
            // checking for the same card 3 times in a row (duplicates are allowed!)
            if actions[i] == actions[i+1] && actions[i+1] == actions[i+2] {
                return Err(DeckError::ActionCardAppearsMoreThanTwice(actions[i]))
            }
        }

        // Verifies if there's elemental resonance within the characters
        // That is, if there are two or three characters of same element
        let resonance: Option<Element> = {
            let el1 = characters[0].element();
            let el2 = characters[1].element();
            let el3 = characters[2].element();

            if el1 == el2 {
                Some(el1)
            } else if el2 == el3 {
                Some(el2)
            } else if el3 == el1 {
                Some(el3)
            } else {
                None
            }
        };

        for i in 0..30 {
            if let ActionCard::Equipment(EquipmentCard::Talent(talent)) = actions[i] {
                let required = talent.character();

                if !characters.contains(&required) {
                    return Err(DeckError::TalentRequiresCharacter(talent))
                }
            }

            if let ActionCard::Event(EventCard::Resonance(elemental)) = actions[i] {
                let required = elemental.element();

                if resonance != Some(required) {
                    return Err(DeckError::ResonanceRequiresAtLeastTwo(elemental))
                }
            }
        }

        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub enum DeckError {
    /// Character cards > 3
    TooManyCharacterCards,
    /// Action cards > 30
    TooManyActionCards,
    /// Character cards < 3 (u8 represents current amount)
    NotEnoughCharacterCards(u8),
    /// Action cards < 30 (u8 represents current amount)
    NotEnoughActionCards(u8),
    /// Talent card is present, but its CharacterCard requirement is not
    TalentRequiresCharacter(TalentCard),
    /// Resonance card is present, but less than 2 characters that match its element
    ResonanceRequiresAtLeastTwo(ElementalResonanceCard),
    /// Deck must include three different character cards
    CharacterAppearsMoreThanOnce(CharacterCard),
    /// Only one or two of the same action card allowed for a deck
    ActionCardAppearsMoreThanTwice(ActionCard),
}

use std::fmt;

impl fmt::Display for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errstr = match self {
            Self::TalentRequiresCharacter(card) => {
                let talent_name = card.name();
                let char_name = card.character().name();

                format!("`{talent_name}` requires `{char_name}` to be present in the deck")
            },
            Self::ResonanceRequiresAtLeastTwo(card) => {
                let res_name = card.name();
                let element = card.element();

                format!("`{res_name}` requires at least two {element:?} characters in the deck")
            }
            Self::CharacterAppearsMoreThanOnce(card) => {
                let char_name = card.name();

                format!("deck contains more than one `{char_name}`")
            },
            Self::ActionCardAppearsMoreThanTwice(card) => {
                let card_name = card.name();

                format!("deck contains more than two `{card_name}`")
            },
            Self::TooManyCharacterCards => "deck has more than three character cards".into(),
            Self::TooManyActionCards => "deck has more than 30 action cards".into(),
            Self::NotEnoughCharacterCards(x) => {
                format!("decks require 3 character cards, only retrieved {x}")
            },
            Self::NotEnoughActionCards(x) => {
                format!("decks require 30 action cards, only retrieved {x}")
            },
        };
        
        write!(f, "{errstr}")
    }
}

impl fmt::Debug for DeckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for DeckError {}

/// Helper struct for iterating over action cards in a [`Deck`]
/// 
/// Can only be created with the [`iter`] and [`iter_unique`] methods
/// 
/// [`iter`]: Deck::iter
/// [`iter_unique`]: Deck::iter_unique
pub struct IterAction<'d> {
    array: &'d [ActionCard; 30],
    index: usize,
    unique: bool,
}

impl Iterator for IterAction<'_> {
    type Item = ActionCard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 30 {
            let card = self.array[self.index];

            if self.unique && self.index < 29 {
                if card == self.array[self.index + 1] {
                    self.index += 2; // Assumes you can only have at most 2 cards
                } else {
                    self.index += 1;
                }
            } else {
                self.index += 1;
            }

            Some(card)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use rand::prelude::*;
    use crate::*;

    const CH: CharacterCard = CharacterCard::Chongyun;
    const XI: CharacterCard = CharacterCard::Xingqiu;
    const FI: CharacterCard = CharacterCard::Fischl;
    const KE: CharacterCard = CharacterCard::Keqing;
    const RA: CharacterCard = CharacterCard::Razor;
    const LA: CharacterCard = CharacterCard::StonehideLawachurl;
    const NO: CharacterCard = CharacterCard::Noelle;
    const CO: CharacterCard = CharacterCard::Collei;
    const AG: CharacterCard = CharacterCard::FatuiPyroAgent;
    const YO: CharacterCard = CharacterCard::Yoimiya;

    const GA: ActionCard = ActionCard::Equipment(EquipmentCard::Artifact(ArtifactCard::GamblersEarrings));
    const LU: ActionCard = ActionCard::Equipment(EquipmentCard::Artifact(ArtifactCard::LuckyDogsSilverCirclet));
    const DE: ActionCard = ActionCard::Equipment(EquipmentCard::Artifact(ArtifactCard::DeepwoodMemories));
    const VI: ActionCard = ActionCard::Equipment(EquipmentCard::Artifact(ArtifactCard::ViridescentVenerer));
    const TH: ActionCard = ActionCard::Equipment(EquipmentCard::Talent(TalentCard::TheScentRemained));
    const AW: ActionCard = ActionCard::Equipment(EquipmentCard::Talent(TalentCard::Awakening));
    const IG: ActionCard = ActionCard::Equipment(EquipmentCard::Talent(TalentCard::IGotYourBack));
    const SH: ActionCard = ActionCard::Equipment(EquipmentCard::Talent(TalentCard::ShakenNotPurred));
    const AQ: ActionCard = ActionCard::Equipment(EquipmentCard::Weapon(WeaponCard::AquilaFavonia));
    const WO: ActionCard = ActionCard::Equipment(EquipmentCard::Weapon(WeaponCard::WolfsGravestone));
    const WH: ActionCard = ActionCard::Equipment(EquipmentCard::Weapon(WeaponCard::WhiteTassel));
    const LO: ActionCard = ActionCard::Event(EventCard::Food(FoodCard::LotusFlowerCrisp));
    const MI: ActionCard = ActionCard::Event(EventCard::Food(FoodCard::MintyMeatRolls));
    const AD: ActionCard = ActionCard::Event(EventCard::Food(FoodCard::AdeptusTemptation));
    const JU: ActionCard = ActionCard::Event(EventCard::Food(FoodCard::JueyunGuoba));
    const LE: ActionCard = ActionCard::Event(EventCard::Normal(NormalEventCard::LeaveItToMe));
    const ST: ActionCard = ActionCard::Event(EventCard::Normal(NormalEventCard::Strategize));
    const MA: ActionCard = ActionCard::Event(EventCard::Normal(NormalEventCard::MasterOfWeaponry));
    const AB: ActionCard = ActionCard::Event(EventCard::Normal(NormalEventCard::AbyssalSummons));
    const HI: ActionCard = ActionCard::Event(EventCard::Resonance(ElementalResonanceCard::HighVoltage));
    const EN: ActionCard = ActionCard::Event(EventCard::Resonance(ElementalResonanceCard::EnduringRock));
    const PA: ActionCard = ActionCard::Support(SupportCard::Companion(CompanionCard::Paimon));
    const LI: ActionCard = ActionCard::Support(SupportCard::Companion(CompanionCard::Liben));
    const TI: ActionCard = ActionCard::Support(SupportCard::Companion(CompanionCard::Timaeus));
    const TU: ActionCard = ActionCard::Support(SupportCard::Companion(CompanionCard::Tubby));
    const NR: ActionCard = ActionCard::Support(SupportCard::Item(ItemCard::NRE));
    const JA: ActionCard = ActionCard::Support(SupportCard::Location(LocationCard::JadeChamber));
    const FA: ActionCard = ActionCard::Support(SupportCard::Location(LocationCard::FavoniusCathedral));
    const DA: ActionCard = ActionCard::Support(SupportCard::Location(LocationCard::DawnWinery));
    const WA: ActionCard = ActionCard::Support(SupportCard::Location(LocationCard::WangshuInn));

    fn iter(characters: &[CharacterCard], actions: &[ActionCard]) -> Vec<Card> {
        characters.iter().map(|x| Card::from(*x))
            .chain(actions.iter().map(|x| Card::from(*x)))
            .collect()
    }

    #[test]
    fn eq_works() {
        let characters = [LA, CO, NO];
        let actions = [ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, EN, DE];

        let deck = iter(&characters, &actions);        
        let original_deck = Deck::from_iter(deck.iter().copied());        
        
        let mut rng = thread_rng();

        for _ in 0..5 {
            let mut shuffled = deck.clone();
            shuffled.shuffle(&mut rng);

            let shuffled_deck = Deck::from_iter(shuffled.iter().copied());

            // Two decks should be considered equal as long as they have the same cards, no
            // matter what order they were declared in
            if original_deck != shuffled_deck {
                panic!("These vec iterators produce different decks:\n{original_deck:?}\n{shuffled_deck:?}")
            }
        }
    }

    #[test]
    fn wrong_amounts() {
        let not_enough_chars = iter(
            &[LA, NO],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, EN, DE]
        );

        let not_enough_actions = iter(
            &[LA, CO, NO],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, TI, AQ, AB, TU, AB, EN, DE]
        );

        let too_many_chars = iter(
            &[LA, CO, NO, CH],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, EN, DE]
        );

        let too_many_actions = iter(
            &[LA, CO, NO],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, LI, TU, IG, TI, AQ, AB, TU, AB, EN, DE]
        );

        assert_eq!(Deck::from_iter(not_enough_chars), Err(DeckError::NotEnoughCharacterCards(2)));
        assert_eq!(Deck::from_iter(not_enough_actions), Err(DeckError::NotEnoughActionCards(29)));
        assert_eq!(Deck::from_iter(too_many_chars), Err(DeckError::TooManyCharacterCards));
        assert_eq!(Deck::from_iter(too_many_actions), Err(DeckError::TooManyActionCards));
    }

    #[test]
    fn talent_error() {
        let talent_missing_character = iter(
            &[FI, RA, AG],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, AQ, DE]
        );

        assert_eq!(
            Deck::from_iter(talent_missing_character),
            Err(DeckError::TalentRequiresCharacter(TalentCard::IGotYourBack))
        );

        // including Noelle in the deck (character for TalentCard::IGotYourBack) fixes it
        let this_works = iter(
            &[FI, NO, AG],
            &[ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, AQ, DE]
        );

        assert!(Deck::from_iter(this_works).is_ok());
    }

    #[test]
    fn resonance_error() {
        let actions = [ST, PA, VI, HI, LU, WO, WH, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, HI, TU, MI, TI, AQ, AB, TU, AB, AQ, DE];

        let resonance_missing_chars = iter(&[FI, CH, XI], &actions);

        assert_eq!(
            Deck::from_iter(resonance_missing_chars),
            Err(DeckError::ResonanceRequiresAtLeastTwo(ElementalResonanceCard::HighVoltage))
        );

        // having 2 or 3 characters of the resonance's element fixes it
        let resonance_with_two   = iter(&[RA, FI, XI], &actions);
        let resonance_with_three = iter(&[KE, RA, FI], &actions);

        assert!(Deck::from_iter(resonance_with_two).is_ok());
        assert!(Deck::from_iter(resonance_with_three).is_ok());

    }

    #[test]
    fn multiple_error() {
        let duplicate_char = iter(
            &[YO, AG, YO],
            &[ST, PA, VI, DE, LU, WO, WH, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, PA, TU, MI, TI, AQ, AB, TU, AB, AQ, DE]
        );

        assert_eq!(
            Deck::from_iter(duplicate_char),
            Err(DeckError::CharacterAppearsMoreThanOnce(CharacterCard::Yoimiya)),
        );

        let triplicate_card = iter(
            &[LA, CO, NO],
            &[ST, PA, VI, TU, LU, WO, WH, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, HI, TU, MI, TI, AQ, AB, TU, AB, AQ, DE]
        );

        assert_eq!(
            Deck::from_iter(triplicate_card),
            Err(DeckError::ActionCardAppearsMoreThanTwice(TU)),
        );
    }

    #[test]
    fn from_exact_works() {
        let characters = [LA, CO, NO];
        let actions = [ST, PA, VI, DA, LU, WO, IG, NR, WA, JU, JA, WA, MI, WH, FA, LE, GA, LO, MA, AD, LI, TU, IG, TI, AQ, AB, TU, AB, EN, DE];

        let card_vec = iter(&characters, &actions);

        assert_eq!(
            Deck::from_iter(card_vec),
            Deck::from_exact(characters, actions),
        );
    }
}