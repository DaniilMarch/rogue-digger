use crate::components::common::*;
use crate::components::level::*;
use crate::prelude::*;
use crate::level_generation::{LevelGenerator};
use rand::Rng;

pub struct  CellularAutomataGenerator<'a> {
    ecs: Option<&'a mut World>,
    fill_percent: f32,
    rng: rand::rngs::ThreadRng,
}


impl<'a> CellularAutomataGenerator<'a> {
    pub fn new(fill_percent: f32) -> Self {
        Self {
            ecs: None,
            fill_percent,
            rng: rand::thread_rng(),
        }
    }

    pub fn generate(&mut self) {
        if let Some(ecs) = &mut self.ecs {
            let mut map = [[false; U_SCREEN_HEIGHT]; U_SCREEN_WIDTH];
            for x in 0..U_SCREEN_WIDTH {
                for y in 0..U_SCREEN_HEIGHT {
                    map[x][y] = if self.rng.gen::<f32>() < self.fill_percent { true } else { false }
                }
            }

            for i in 0..1 {
                for x in 0..U_SCREEN_WIDTH {
                    for y in 0..U_SCREEN_HEIGHT {
                        let neighbours_count = CellularAutomataGenerator::get_neighbours_count(&mut map, x as i32, y as i32);
                        if neighbours_count > 4 {
                            map[x][y] = true;
                        } else if neighbours_count < 4 {
                            map[x][y] = false;
                        }
                    }
                }
            }
    
            for x in 0..U_SCREEN_WIDTH {
                for y in 0..U_SCREEN_HEIGHT {
                    if map[x][y] {
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

    fn get_neighbours_count(map: &mut [[bool; U_SCREEN_HEIGHT]; U_SCREEN_WIDTH], x: i32, y: i32) -> i32 {
        let mut neighbours_count = 0;
        for neighbour_x in (x - 1)..=(x + 1) {
            for neighbour_y in (y - 1)..=(y + 1) {
                if neighbour_x >= 0 && neighbour_x < SCREEN_WIDTH && neighbour_y >=0 && neighbour_y < SCREEN_HEIGHT {
                    if (neighbour_x != x || neighbour_y != y) {
                        neighbours_count += map[neighbour_x as usize][neighbour_y as usize] as i32;
                    }
                } else {
                    neighbours_count += 1;
                }
            }
        }

        neighbours_count
    }
}

impl<'a > LevelGenerator<'a> for CellularAutomataGenerator<'a> {
    fn register_world(&mut self, ecs: &'a mut World) {
        self.ecs = Some(ecs);
    }
}
