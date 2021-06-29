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

#[derive(Clone, Copy)]
pub struct DroppingLoot {
    pub add_loot: fn(&mut CommandBuffer, Point) -> (),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeathIntention {
    pub entity: Entity,
}
