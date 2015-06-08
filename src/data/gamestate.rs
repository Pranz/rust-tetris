extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use data::shapes::{l_block, BlockType, Block, square_block, l_block_mirrored, BLOCK_SIZE};
use data::colors::*;

pub const WIDTH : usize = 10;
pub const HEIGHT : usize = 20;


pub struct GameState {
	pub map               : [[bool; HEIGHT]; WIDTH],
    pub frames_until_move : u16,
    pub frames_passed     : u16,
    pub block             : &'static [Block],
    pub block_rotation    : u8,
    pub block_x           : u8,
    pub block_y           : u8,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
    	    map : [[false; HEIGHT]; WIDTH],
            frames_until_move : 60,
            frames_passed     : 0,
            block             : &l_block,
            block_rotation    : 0,
            block_x           : 0,
            block_y           : 0,
		};
		state
	}

	pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 16.0);

        gl.draw(args.viewport(), |c, g| {
            clear(BLACK, g);

            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    if self.map[i][j] {
                    	let transform = c.transform.trans(i as f64 * 16.0, j as f64 * 16.0);
                    	rectangle(WHITE, square, transform, g);
                    }
                }
            }

            for i in 0..BLOCK_SIZE {
                for j in 0..BLOCK_SIZE {
                    if self.block[self.block_rotation as usize][i as usize][j as usize] {
                        let transform = c.transform.trans((i + self.block_x) as f64 * 16.0, (j + self.block_y) as f64 * 16.0);
                        rectangle(WHITE, square, transform, g);
                    }
                }
            }
        });
    }
    
    pub fn update(&mut self, args: &UpdateArgs) {
        self.frames_passed += 1;
        if self.frames_passed == self.frames_until_move {
            self.block_y += 1;
            self.frames_passed = 0;
            self.next_rotation();
        }
    }

    pub fn next_rotation(&mut self) {
        self.block_rotation = (self.block_rotation + 1) % self.block.len() as u8;
    }
}


