use crate::prelude::*;
use crate::resources::turn_state::*;

#[system]
pub fn turn_end(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        TurnState::PlayerInput => return,
        TurnState::PlayerTurn => TurnState::NpcTurn,
        TurnState::NpcTurn => TurnState::PlayerInput,
    };
    *turn_state = new_state;
}
