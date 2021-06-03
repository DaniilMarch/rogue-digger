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
}

use boot::{
    game_state::State,
    initial_level::generate_initial_level,
    terminal::build_terminal,
};
use components::common::{Player, Render};
use prelude::*;
use resources::turn_state::TurnState;

fn main() -> BError {
    let ctx = build_terminal();
    let mut state = State::new();
    state.resources.insert(TurnState::PlayerInput);
    generate_initial_level(&mut state.ecs);
    state.ecs.push((
        Player,
        Render {
            color: ColorPair::new(RED, BLACK),
            glyph: '@' as u16,
        },
        Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
    ));
    main_loop(ctx, state)
}
