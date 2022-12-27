use super::Price;
use crate::*;

macro_rules! impl_method {
    ($($type:ident : $article:ident $name:ident ,)+) => {
        $(
            #[doc = "Attempts to parse `self` as "]
            #[doc = stringify!($article)]
            #[doc = stringify!($name)]
            #[doc = "card. Returns None if this card is of another type/subtype"]
            fn $name(&self) -> Option<$type> { None }
        )+
    };
}

pub trait PlayingCard: sealed::Trait {
    /// The display name of the card
    fn name(&self) -> &'static str;

    /// How much it costs to play this card ingame
    fn cost(&self) -> CardCost;

    /// Returns the price for buying this card in Prince's shop (in Lucky Coins)
    /// 
    /// If this card is not obtainable from the shop (such as [talent cards]
    /// or [Paimon]), returns [`None`]
    /// 
    /// [talent cards]: crate::TalentCard
    /// [Paimon]: crate::CompanionCard::Paimon
    fn shop_price(&self) -> Option<Price>;

    impl_method!(
        EquipmentCard: an equipment,
        EventCard: an event,
        SupportCard: a support,
        ArtifactCard: an artifact,
        TalentCard: a talent,
        WeaponCard: a weapon,
        FoodCard: a food,
        CompanionCard: a companion,
        ItemCard: an item,
        LocationCard: a location,
        ElementalResonanceCard: a resonance,
    );

    /// Attempts to parse `self` as a normal event card (non-food and non-resonance).
    /// Returns None if this card is of another type/subtype
    fn normal_event(&self) -> Option<NormalEventCard> { None }

}

mod sealed {
    use crate::*;
    pub trait Trait {}

    macro_rules! impl_trait {
        ($($card:ident)+) => {
            $(
                impl Trait for $card {}
            )+
        };
    }

    impl_trait!(ActionCard
        EquipmentCard ArtifactCard TalentCard WeaponCard
        EventCard FoodCard NormalEventCard ElementalResonanceCard
        SupportCard CompanionCard ItemCard LocationCard
    );
}