/// The seven elements within Teyvat
/// 
/// # Example
/// 
/// ```
/// # use genius_invokation::{CharacterCard, Element};
/// assert_eq!(CharacterCard::Fischl.element(), Element::Electro);
/// ```
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Cryo,
    Hydro,
    Electro,
    Geo,
    Dendro,
    Anemo,
    Pyro,
}