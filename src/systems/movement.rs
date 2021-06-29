use crate::components::common::*;
use crate::resources::camera::Camera;

//Movement system queries all entities with MovementIntention component which container reference to one who wants to move
//And where they want to move
//After we apply movement, we remove this entity
//Changing components and removing entities is done with command buffer and not with ecs
#[system]
#[read_component(MovementIntention)]
#[write_component(MovementIntention)]
#[read_component(Player)]
pub fn movement(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] camera: &mut Camera) {
    let mut with_movement_intent = <(Entity, &MovementIntention)>::query();
    for (entity, wants_to_move) in with_movement_intent.iter(ecs) {
        commands.add_component(wants_to_move.entity, wants_to_move.destination);
        if ecs
            .entry_ref(wants_to_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok()
        {
            camera.on_player_move(wants_to_move.destination);
        }
        commands.remove(*entity);
    }
}
