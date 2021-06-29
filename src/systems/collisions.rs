use crate::components::common::MovementIntention;
use crate::components::level::Wall;
use crate::prelude::*;

#[system]
#[read_component(MovementIntention)]
#[read_component(Point)]
#[read_component(Wall)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &MovementIntention)>::query();
    let mut walls = <(&Wall, &Point)>::query();
    //TODO: improve iterations
    for (entity, movement_intention) in movers.iter(ecs) {
        for (_, point) in walls.iter(ecs) {
            if point.x == movement_intention.destination.x
                && point.y == movement_intention.destination.y
            {
                commands.remove(*entity);
                break;
            }
        }
    }
}
