use crate::prelude::*;

pub fn build() -> BTerm {
    let context = BTermBuilder::new()
        .with_title("Straight to Hell")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("assets")
        .with_font("terminal8x8.png", 8, 8)
        .with_sparse_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .with_sparse_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .with_sparse_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "terminal8x8.png")
        .build()
        .expect("Terminal build error");

    context
}
