use crate::components::{common::*, level::*};

#[system]
#[read_component(Floor)]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
#[read_component(NPC)]
pub fn render(ecs: &SubWorld) {
    let mut floor_batch = DrawBatch::new();
    floor_batch.target(0);
    let mut floor_tiles = <(&Point, &Render)>::query().filter(component::<Floor>());
    let mut rendered_tiles = 0;
    for (point, render) in floor_tiles.iter(ecs) {
        floor_batch.set(point.clone(), render.color, render.glyph);
        rendered_tiles += 1;
    }
    floor_batch.submit(0).expect("Render error");

    let mut walls_batch = DrawBatch::new();
    walls_batch.target(1);
    let mut wall_tiles = <(&Point, &Render)>::query().filter(component::<Wall>());
    for (point, render) in wall_tiles.iter(ecs) {
        walls_batch.set(point.clone(), render.color, render.glyph);
        rendered_tiles += 1;
    }
    walls_batch.submit(rendered_tiles).expect("Render error");

    let mut actors_batch = DrawBatch::new();
    actors_batch.target(2);
    let mut actors =
        <(&Point, &Render)>::query().filter(component::<Player>() | component::<NPC>());
    for (point, render) in actors.iter(ecs) {
        actors_batch.set(point.clone(), render.color, render.glyph);
    }
    actors_batch.submit(rendered_tiles).expect("Render error");
}
