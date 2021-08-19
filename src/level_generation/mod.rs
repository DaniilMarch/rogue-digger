use crate::prelude::*;
pub mod initial_test_generator;
pub mod circle_room_generator;

pub trait LevelGenerator<'a > {
    fn register_world(&mut self, world: &'a mut World);
}