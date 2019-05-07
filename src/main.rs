extern crate piston_window;

use piston_window::*;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct GameOfLife {
    pixel_size: f64,
    color: [f32; 4],
    state: [[bool; 100]; 100],
}

impl GameOfLife {
    fn render(&mut self, event: &Event, args: &RenderArgs, window: &mut PistonWindow) {
        println!("render");
        let pixel = rectangle::square(0.0, 0.0, self.pixel_size);

        window.draw_2d(event, |c, g| {
            for i in 0..100 {
                for j in 0..100 {
                    let pixel_state = self.state[i][j];
                    let pixel_color = if pixel_state { BLACK } else { WHITE };
                    let pixel_x: f64 = i as f64 * self.pixel_size + self.pixel_size;
                    let pixel_y: f64 = j as f64 * self.pixel_size + self.pixel_size;
                    let pixel_transform = c.transform.trans(pixel_x, pixel_y);
                    rectangle(pixel_color, pixel, pixel_transform, g);
                }
            }
        });
    }

    fn update(&mut self, update: &UpdateArgs) {
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
        color: RED,
        state: new_state,
    };

    let speed = 1;
    let mut events = Events::new(EventSettings::new().ups(speed).max_fps(speed));
    while let Some(event) = events.next(&mut window) {
        if let Some(r) = event.render_args() {
            app.render(&event, &r, &mut window);
        }

        if let Some(u) = event.update_args() {
            app.update(&u);
        }
    }
}
