use crate::prelude::*;
mod movement;
mod player_input;
mod render;
mod turn_end;

pub fn build_player_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(render::render_system())
        .add_system(turn_end::turn_end_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(render::render_system())
        .add_system(turn_end::turn_end_system())
        .build()
}

pub fn build_npc_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(render::render_system())
        .add_system(turn_end::turn_end_system())
        .build()
}
