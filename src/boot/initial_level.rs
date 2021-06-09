use crate::components::common::*;
use crate::components::level::*;
use crate::prelude::*;

pub fn generate(ecs: &mut World) {
    for x in 0..=SCREEN_WIDTH {
        for y in 0..=SCREEN_HEIGHT {
            ecs.push((
                Point::new(x, y),
                Floor,
                Render {
                    color: ColorPair::new(YELLOW, BLACK),
                    glyph: '.' as u16,
                },
            ));
        }
    }

    ecs.push((
        Point::new(SCREEN_WIDTH / 2 - 1, SCREEN_HEIGHT / 2 - 1),
        Wall,
        Render {
            color: ColorPair::new(WHITE, WHITE),
            glyph: '+' as u16,
        },
    ));
}
