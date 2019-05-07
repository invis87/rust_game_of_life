extern crate piston_window;

use piston_window::*;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct GameOfLife {
    pixel_size: f64,
    state: [[bool; 100]; 100],
}

impl GameOfLife {
    fn pixel_coord(x_id: usize, y_id: usize, size: f64) -> (f64, f64) {
        let x_coord = x_id as f64 * size + size;
        let y_coord = y_id as f64 * size + size;
        (x_coord, y_coord)
    }

    fn render(&mut self, event: &Event, window: &mut PistonWindow) {
        println!("render");
        let pixel = rectangle::square(0.0, 0.0, self.pixel_size);

        window.draw_2d(event, |c, g| {
            for i in 0..100 {
                for j in 0..100 {
                    let pixel_state = self.state[i][j];
                    let pixel_color = if pixel_state { BLACK } else { WHITE };
                    let (pixel_x, pixel_y) = GameOfLife::pixel_coord(i, j, self.pixel_size);
                    let pixel_transform = c.transform.trans(pixel_x, pixel_y);
                    rectangle(pixel_color, pixel, pixel_transform, g);
                }
            }
        });
    }

    fn update(&mut self) {
        println!("update");
        self.state[1][1] = !self.state[1][1];
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Game of life!", [640, 640])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut new_state = [[false; 100]; 100];
    for i in 0..100 {
        for j in 0..100 {
            new_state[i][j] = (i + j) % 2 == 0;
        }
    }

    let mut app = GameOfLife {
        pixel_size: 5.0,
        state: new_state,
    };

    let speed = 1;
    let mut events = Events::new(EventSettings::new().ups(speed).max_fps(speed));
    while let Some(event) = events.next(&mut window) {
        if let Some(_) = event.render_args() {
            app.render(&event, &mut window);
        }

        if let Some(_) = event.update_args() {
            app.update();
        }
    }
}


#[test]
fn pixel_coordinations_test() {
    assert_eq!(GameOfLife::pixel_coord(10, 10, 2.0), (22.0, 22.0));
    assert_eq!(GameOfLife::pixel_coord(0, 0, 2.0), (2.0, 2.0));
}