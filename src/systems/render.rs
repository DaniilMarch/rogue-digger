use crate::components::{common::*, level::*};
use crate::resources::camera::Camera;

#[system]
#[read_component(Floor)]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(Player)]
#[read_component(NPC)]
pub fn render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut floor_batch = DrawBatch::new();
    floor_batch.target(0);
    let mut floor_tiles = <(&Point, &Render)>::query().filter(component::<Floor>());
    let mut rendered_tiles = 0;
    let offset = Point::new(camera.left_x, camera.top_y);
    for (point, render) in floor_tiles.iter(ecs) {
        floor_batch.set(point.clone() - offset, render.color, render.glyph);
        rendered_tiles += 1;
    }
    floor_batch.submit(0).expect("Render error");

    let mut walls_batch = DrawBatch::new();
    walls_batch.target(1);
    let mut wall_tiles = <(&Point, &Render)>::query().filter(component::<Wall>());
    for (point, render) in wall_tiles.iter(ecs) {
        walls_batch.set(point.clone() - offset, render.color, render.glyph);
        rendered_tiles += 1;
    }
    walls_batch.submit(rendered_tiles).expect("Render error");

    let mut items_batch = DrawBatch::new();
    items_batch.target(2);
    let mut items = <(&Point, &Render)>::query().filter(component::<Item>());
    for (point, render) in items.iter(ecs) {
        println!("drawing item");
        items_batch.set(point.clone() - offset, render.color, render.glyph);
        rendered_tiles += 1;
    }
    items_batch.submit(rendered_tiles).expect("Render error");

    let mut actors_batch = DrawBatch::new();
    actors_batch.target(3);
    let mut actors =
        <(&Point, &Render)>::query().filter(component::<Player>() | component::<NPC>());
    for (point, render) in actors.iter(ecs) {
        actors_batch.set(point.clone() - offset, render.color, render.glyph);
    }
    actors_batch.submit(rendered_tiles).expect("Render error");
}
