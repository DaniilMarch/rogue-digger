#![warn(clippy::all, clippy::pedantic)]
mod boot;
mod components;
mod resources;
mod systems;
mod level_generation;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const U_SCREEN_WIDTH: usize = SCREEN_WIDTH as usize;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const U_SCREEN_HEIGHT: usize = SCREEN_HEIGHT as usize;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use boot::{game_state::State, terminal};
use components::common::{Player, Render};
use prelude::*;
use resources::turn_state::TurnState;
use level_generation::initial_test_generator::InitialTestGenerator;
use level_generation::circle_room_generator::CircleRoomGenerator;
use level_generation::cellular_automata_generator::CellularAutomataGenerator;
use level_generation::LevelGenerator;

fn main() -> BError {
    let ctx = terminal::build();
    let mut state = State::new();
    state.resources.insert(TurnState::PlayerInput);
    let mut generator = CellularAutomataGenerator::new(0.5);
    generator.register_world(&mut state.ecs);
    generator.generate();
    state.ecs.push((
        Player,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: '@' as u16,
        },
        Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
    ));
    state
        .resources
        .insert(resources::camera::Camera::new(Point::new(
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
        )));
    main_loop(ctx, state)
}
