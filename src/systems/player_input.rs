use crate::components::common::*;
use crate::resources::turn_state::*;

//Player input system reads key and then creates entity with MovementIntention component
//Since legion wont allow single component entities, we add empty tuple as other component
//Movement intention stores where player or else wants to move and entity - who wants to move
#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        let mut player_pos = <(Entity, &mut Point)>::query().filter(component::<Player>());
        player_pos.iter_mut(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands.push((
                (),
                MovementIntention {
                    destination,
                    entity: *entity,
                },
            ));
        });

        *turn_state = TurnState::PlayerTurn;
    }
}
