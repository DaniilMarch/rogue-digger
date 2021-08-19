use crate::components::common::{DeathIntention, MovementIntention, Player, Render};
use crate::components::level::Breakable;
use crate::prelude::*;

#[system]
#[read_component(Breakable)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(MovementIntention)]
#[write_component(Render)]
#[write_component(Breakable)]
pub fn breakable(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <&MovementIntention>::query();
    let mut walls = <(&Breakable, &Point, &Render)>::query();
    //TODO: improve iterations
    let mut points: Vec<Point> = Vec::new();
    for movement_intention in movers.iter(ecs) {
        for (_, point, _) in walls.iter(ecs) {
            if point.x == movement_intention.destination.x
                && point.y == movement_intention.destination.y
            {
                points.push(*point);
                break;
            }
        }
    }

    let mut breaking = <(Entity, &mut Breakable, &Point, &mut Render)>::query();
    for matching_point in points {
        for (entity, breakable, point, render) in breaking.iter_mut(ecs) {
            if point.x == matching_point.x && point.y == matching_point.y {
                breakable.health -= 20;
                for (treshold, glyph, color_pair) in breakable.render_set.iter() {
                    if breakable.health <= 0 {
                        commands.push(((), DeathIntention { entity: *entity }));
                        break;
                    }
                    if breakable.health >= *treshold {
                        render.glyph = *glyph;
                        render.color = *color_pair;
                        break;
                    }
                }
            }
        }
    }
}
