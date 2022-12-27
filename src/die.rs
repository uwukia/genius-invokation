use crate::Element;

/// An elemental octet die 
#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Die {
    Omni,
    Element(Element)
}