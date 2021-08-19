use crate::components::common::*;
use crate::components::level::*;
use crate::prelude::*;
use crate::level_generation::{LevelGenerator};

pub struct  CircleRoomGenerator<'a> {
    ecs: Option<&'a mut World>,
    radius: i32,
}


impl<'a> CircleRoomGenerator<'a> {
    pub fn new(radius: i32) -> Self {
        Self {
            ecs: None,
            radius,
        }
    }

    pub fn generate_circle(&mut self) {
        if let Some(ecs) = &mut self.ecs {
            let center_x = SCREEN_WIDTH / 2;
            let center_y = SCREEN_HEIGHT / 2;

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

            for x in 0..=SCREEN_WIDTH {
                for y in 0..=SCREEN_HEIGHT {
                    let distance_from_center = (((x - center_x).pow(2) + (y - center_y).pow(2)) as f32).powf(0.5);
                    if distance_from_center > self.radius as f32 {
                        ecs.push((
                            Point::new(x, y),
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
                }
            }
        }
    }
}

impl<'a > LevelGenerator<'a> for CircleRoomGenerator<'a> {
    fn register_world(&mut self, ecs: &'a mut World) {
        self.ecs = Some(ecs);
    }
}
