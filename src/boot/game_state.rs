use crate::prelude::*;
use crate::resources::turn_state::*;
use crate::systems::*;

pub struct State {
    pub ecs: World,
    pub resources: Resources,
    pub player_input_systems: Schedule,
    pub player_turn_systems: Schedule,
    pub npc_turn_systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        let ecs = World::default();
        let mut resources = Resources::default();
        resources.insert(TurnState::PlayerInput);
        Self {
            ecs,
            resources,
            player_input_systems: build_player_input_scheduler(),
            player_turn_systems: build_player_turn_scheduler(),
            npc_turn_systems: build_npc_turn_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        self.resources.insert(ctx.key);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::PlayerInput => self
                .player_input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_turn_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::NpcTurn => self
                .npc_turn_systems
                .execute(&mut self.ecs, &mut self.resources),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}
