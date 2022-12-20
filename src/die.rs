use crate::Element;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Die {
    Onmi,
    Element(Element)
}