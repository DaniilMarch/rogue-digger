#![warn(clippy::all, clippy::pedantic)]
mod boot;
mod components;
mod resources;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use boot::{game_state::State, initial_level, terminal};
use components::common::{Player, Render};
use prelude::*;
use resources::turn_state::TurnState;

fn main() -> BError {
    let ctx = terminal::build();
    let mut state = State::new();
    state.resources.insert(TurnState::PlayerInput);
    initial_level::generate(&mut state.ecs);
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
