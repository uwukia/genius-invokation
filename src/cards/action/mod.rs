pub use equip::*;
mod equip;

pub use event::*;
mod event;

pub use support::*;
mod support;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum ActionCard {
    Equipment(EquipmentCard),
    Support(SupportCard),
    Event(EventCard),
}