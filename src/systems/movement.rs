use crate::components::common::*;

//Movement system queries all entities with MovementIntention component which container reference to one who wants to move
//And where they want to move
//After we apply movement, we remove this entity
//Changing components and removing entities is done with command buffer and not with ecs
#[system]
#[read_component(MovementIntention)]
#[write_component(MovementIntention)]
pub fn movement(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut with_movement_intent = <(Entity, &MovementIntention)>::query();
    for (entity, wants_to_move) in with_movement_intent.iter_mut(ecs) {
        commands.add_component(wants_to_move.entity, wants_to_move.destination);
        commands.remove(*entity);
    }
}
