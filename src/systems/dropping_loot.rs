use crate::components::common::{DeathIntention, DroppingLoot};
use crate::prelude::*;

#[system]
#[read_component(DroppingLoot)]
#[read_component(DeathIntention)]
#[read_component(Point)]
pub fn dropping_loot(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut dying_entities = <(&DeathIntention)>::query();
    for (death_intention) in dying_entities.iter(ecs) {
        if let Ok(drop_component) = ecs
            .entry_ref(death_intention.entity)
            .unwrap()
            .get_component::<DroppingLoot>()
        {
            if let Ok(position_component) = ecs
            .entry_ref(death_intention.entity)
            .unwrap()
            .get_component::<Point>() {
                ((*drop_component).add_loot)(commands, position_component.clone());
            }
        }
    }
}
