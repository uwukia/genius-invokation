use crate::{CharacterCard, ActionCard, Card, EquipmentCard, TalentCard};
use super::cards::CardOrd;

#[derive(Debug, Clone)]
pub struct Deck {
    characters: [CharacterCard; 3],
    actions: [ActionCard; 30],
}

impl Deck {
    pub fn characters(&self) -> &[CharacterCard; 3] {
        &self.characters
    }

    pub fn action_cards(&self) -> &[ActionCard; 30] {
        &self.actions
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

        for i in 0..30 {
            if let ActionCard::Equipment(EquipmentCard::Talent(talent)) = actions[i] {
                let required = talent.character();

                if !characters.contains(&required) {
                    return Err(DeckError::TalentRequiresCharacter(talent))
                }
            }
        }

        Ok(())
    }
}



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