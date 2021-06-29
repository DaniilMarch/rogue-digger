use crate::components::common::DeathIntention;
use crate::prelude::*;

#[system]
#[write_component(DeathIntention)]
#[read_component(DeathIntention)]
pub fn cleanup(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut dying = <(Entity, &DeathIntention)>::query();
    for (entity, death_intention) in dying.iter_mut(ecs) {
        commands.remove(*entity);
        commands.remove(death_intention.entity);
    }
}
