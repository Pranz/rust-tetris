extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::event::*;
use piston::input::Key;
use piston::window::WindowSettings;

use data::colors::*;
use data::shapes::{l_block, BlockType, Block, square_block, l_block_mirrored, BLOCK_SIZE, block_intersects, imprint_block};

pub const WIDTH : usize = 10;
pub const HEIGHT: usize = 20;


pub struct GameState {
	pub map              : [[bool; HEIGHT]; WIDTH],
    pub frames_until_move: u16,
    pub frames_passed    : u16,
    pub block            : &'static [Block],
    pub block_rotation   : u8,
    pub block_x          : i16,
    pub block_y          : i16,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
    	    map: [[false; HEIGHT]; WIDTH],
            frames_until_move: 60,
            frames_passed    : 0,
            block            : &l_block,
            block_rotation   : 0,
            block_x          : 0,
            block_y          : 0,
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
                        let transform = c.transform.trans((i as i16 + self.block_x) as f64 * 16.0, (j as i16 + self.block_y) as f64 * 16.0);
                        rectangle(WHITE, square, transform, g);
                    }
                }
            }
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.frames_passed += 1;
        if self.frames_passed == self.frames_until_move {
            self.frames_passed = 0;
            if block_intersects(&self, self.block_x as i16, self.block_y as i16 + 1) {
                let (x, y) = (self.block_x, self.block_y);
                imprint_block(self, x as u8, y as u8);
                self.block_x = 2;
                self.block_y = 0;
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Right => self.move_block(1, 0),
            Key::Left  => self.move_block(-1, 0),
            Key::Up    => self.next_rotation(),
            _ => {},
        }
    }

    pub fn next_rotation(&mut self) {
        self.block_rotation = (self.block_rotation + 1) % self.block.len() as u8;
    }

    pub fn move_block(&mut self, dx: i16, dy: i16) {
        if !block_intersects(&self, self.block_x + dx, self.block_y + dy) {
            self.block_x += dx;
            self.block_y += dy;
        }
    }
}
