#![deny(bare_trait_objects)]
extern crate piston_window;

use piston_window::*;
use std::ops::Range;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const FIELD_SIZE: usize = 100;

pub struct GameOfLife {
    state: [bool; 100 * 100],
}

impl Default for GameOfLife {
    fn default() -> Self {
        let mut state = [false; FIELD_SIZE * FIELD_SIZE];
        for i in 0..FIELD_SIZE {
            for j in 0..FIELD_SIZE {
                let pixel_index = GameOfLife::pixel_index(i, j);
                state[pixel_index] = (i + j) % 2 == 0;
            }
        }
        GameOfLife { state }
    }
}

struct Lol(usize);
impl Lol {
    fn custom_check_add(&self, rhs: usize) -> Option<usize> {
        if self.0 >= FIELD_SIZE - 1 {
            None
        } else {
            Some(self.0 + rhs)
        }
    }
}

struct NeighborsIterator {
    array: [Option<usize>; 8],
    index: usize,
}
impl NeighborsIterator {
    fn new(array: [Option<usize>; 8]) -> NeighborsIterator {
        NeighborsIterator { array, index: 0 }
    }
}
impl Iterator for NeighborsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.index == 8 {
            None
        } else {
            let curr_elem = self.array[self.index];
            self.index += 1;
            if curr_elem.is_none() {
                self.next()
            } else {
                curr_elem
            }
        }
    }
}

impl GameOfLife {
    fn safe_coord_operation(x: Option<usize>, y: Option<usize>) -> Option<usize> {
        if let (Some(real_x), Some(real_y)) = (x, y) {
            Some(GameOfLife::pixel_index(real_x, real_y))
        } else {
            None
        }
    }

    fn coord_neighbors(x: usize, y: usize) -> NeighborsIterator {
        let res_new = [
            GameOfLife::safe_coord_operation(x.checked_sub(1), Some(y)),
            GameOfLife::safe_coord_operation(x.checked_sub(1), Lol(y).custom_check_add(1)),
            GameOfLife::safe_coord_operation(x.checked_sub(1), y.checked_sub(1)),
            GameOfLife::safe_coord_operation(Lol(x).custom_check_add(1), Some(y)),
            GameOfLife::safe_coord_operation(
                Lol(x).custom_check_add(1),
                Lol(y).custom_check_add(1),
            ),
            GameOfLife::safe_coord_operation(Lol(x).custom_check_add(1), y.checked_sub(1)),
            GameOfLife::safe_coord_operation(Some(x), y.checked_sub(1)),
            GameOfLife::safe_coord_operation(Some(x), Lol(y).custom_check_add(1)),
        ];

        NeighborsIterator::new(res_new)
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> usize {
        let neighbors = GameOfLife::coord_neighbors(x, y);
        let mut alive_counter = 0;
        for neighbor in neighbors {
            if self.state[neighbor] {
                alive_counter += 1
            }
        }

        alive_counter
    }

    fn tick(&mut self) {
        let mut new_state = [false; FIELD_SIZE * FIELD_SIZE];
        for i in 0..FIELD_SIZE {
            for j in 0..FIELD_SIZE {
                let pixel_state = self.pixel_state(i, j);
                let alive_neighbors = self.alive_neighbors(i, j);

                //dead cell
                if !pixel_state && alive_neighbors == 3 {
                    new_state[GameOfLife::pixel_index(i, j)] = true;
                }

                //alive cell
                if pixel_state && !(alive_neighbors == 2 || alive_neighbors == 3) {
                    new_state[GameOfLife::pixel_index(i, j)] = false;
                }
            }
        }

        self.state = new_state;
    }

    fn pixel_state(&self, x: usize, y: usize) -> bool {
        self.state[GameOfLife::pixel_index(x, y)]
    }

    fn pixel_index(x: usize, y: usize) -> usize {
        x * FIELD_SIZE + y
    }
}

struct App {
    pixel_size: f64,
    speed: u64,
    game: GameOfLife,
}

impl App {
    fn new(speed: u64, pixel_size: f64) -> App {
        App {
            speed: 20,
            pixel_size: 5.0,
            game: Default::default(),
        }
    }

    fn pixel_coord(&self, x_id: usize, y_id: usize) -> (f64, f64) {
        let x_coord = x_id as f64 * self.pixel_size + self.pixel_size;
        let y_coord = y_id as f64 * self.pixel_size + self.pixel_size;
        (x_coord, y_coord)
    }

    fn render(&self, event: &Event, window: &mut PistonWindow) {
        let pixel = rectangle::square(0.0, 0.0, self.pixel_size);

        window.draw_2d(event, |c, g| {
            for i in 0..FIELD_SIZE {
                for j in 0..FIELD_SIZE {
                    let pixel_state = self.game.pixel_state(i, j);
                    let pixel_color = if pixel_state { WHITE } else { BLACK };
                    let (pixel_x, pixel_y) = self.pixel_coord(j, i);
                    let pixel_transform = c.transform.trans(pixel_x, pixel_y);
                    rectangle(pixel_color, pixel, pixel_transform, g);
                }
            }
        });
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Game of life!", [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(1, 5.0);

    let mut events = Events::new(EventSettings::new().ups(app.speed).max_fps(app.speed));
    while let Some(event) = events.next(&mut window) {
        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
        }

        if let Some(_) = event.update_args() {
            app.game.tick();
        }
    }
}

#[test]
fn test_neighbors() {
    assert_eq!(
        GameOfLife::coord_neighbors(0, 0).sort(),
        vec![1, 100, 101].sort()
    );
    assert_eq!(
        GameOfLife::coord_neighbors(1, 1).sort(),
        vec![0, 1, 2, 100, 102, 200, 201, 202].sort()
    );
    assert_eq!(
        GameOfLife::coord_neighbors(99, 99).sort(),
        vec![9898, 9899, 9998].sort()
    );
}
