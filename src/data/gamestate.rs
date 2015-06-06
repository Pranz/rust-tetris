extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const WIDTH : usize = 10;
const HEIGHT : usize = 20;

use data::colors::*;

pub struct GameState {
	map : [[bool; HEIGHT]; WIDTH],
}

impl GameState {
	pub fn new() -> Self {
		let mut state = GameState {
			map : [[false; HEIGHT]; WIDTH],
		};
		state.map[3][4] = true;
		state.map[3][5] = true;
		state
	}
	pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 16.0);

        gl.draw(args.viewport(), |c, g| {
            clear(WHITE, g);

            // Draw a box rotating around the middle of the screen.
            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    if self.map[i][j] {
                    	let transform = c.transform.trans(i as f64 * 16.0, j as f64 * 16.0);
                    	rectangle(BLUE, square, transform, g);
                    }
                }
            }
        });
    }
}