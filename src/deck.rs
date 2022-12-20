use crate::{CharacterCard, ActionCard};

#[derive(Debug, Clone)]
pub struct Deck {
    characters: [CharacterCard; 3],
    actions: [ActionCard; 30],
}