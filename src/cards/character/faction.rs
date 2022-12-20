/// I honestly didn't even know this "information" about a card existed until
/// I decided to write this crate
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Faction {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fatui,
    Monster
}