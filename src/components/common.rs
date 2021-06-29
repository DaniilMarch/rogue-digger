pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NPC;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementIntention {
    pub destination: Point,
    pub entity: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DroppingLoot;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeathIntention {
    pub entity: Entity,
}
