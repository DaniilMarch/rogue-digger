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
        Breakable {
            health: 100,
            render_set: vec![
                (100, to_cp437('█'), ColorPair::new(WHITE, BLACK)),
                (75, to_cp437('▓'), ColorPair::new(WHITE, BLACK)),
                (50, to_cp437('▒'), ColorPair::new(WHITE, BLACK)),
                (25, to_cp437('░'), ColorPair::new(WHITE, BLACK)),
                (0, to_cp437('░'), ColorPair::new(WHITE, BLACK)),
            ],
        },
    ));
    ecs.push((
        Point::new(SCREEN_WIDTH / 2 - 2, SCREEN_HEIGHT / 2 - 2),
        Wall,
        Render {
            color: ColorPair::new(WHITE, WHITE),
            glyph: '+' as u16,
        },
        Breakable {
            health: 100,
            render_set: vec![
                (100, to_cp437('█'), ColorPair::new(WHITE, BLACK)),
                (75, to_cp437('▓'), ColorPair::new(WHITE, BLACK)),
                (50, to_cp437('▒'), ColorPair::new(WHITE, BLACK)),
                (25, to_cp437('░'), ColorPair::new(WHITE, BLACK)),
                (0, to_cp437('░'), ColorPair::new(WHITE, BLACK)),
            ],
        },
    ));
}
