use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Floor;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Wall;

#[derive(Clone, PartialEq, Debug)]
pub struct Breakable {
    pub health: i32,
    pub render_set: Vec<(i32, FontCharType, ColorPair)>,
}