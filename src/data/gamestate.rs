use opengl_graphics::GlGraphics;
use piston::event::*;
use rand::{self,Rand};

use super::colors;
use super::shapes::{data,BlockType,BLOCK_SIZE,block_intersects,imprint_block};

pub const WIDTH : usize = 10;
pub const HEIGHT: usize = 20;

pub struct GameState {
	pub map              : [[bool; HEIGHT]; WIDTH],
    pub frames_until_move: u16,
    pub frames_passed    : u16,
    pub block            : &'static [data::Block],
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
            block            : &data::L,
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
            clear(colors::BLACK, g);

            for i in 0..WIDTH {
                for j in 0..HEIGHT {
                    if self.map[i][j] {
                    	let transform = c.transform.trans(i as f64 * 16.0, j as f64 * 16.0);
                    	rectangle(colors::WHITE, square, transform, g);
                    }
                }
            }

            for i in 0..BLOCK_SIZE {
                for j in 0..BLOCK_SIZE {
                    if self.block[self.block_rotation as usize][i as usize][j as usize] {
                        let transform = c.transform.trans((i as i16 + self.block_x) as f64 * 16.0, (j as i16 + self.block_y) as f64 * 16.0);
                        rectangle(colors::WHITE, square, transform, g);
                    }
                }
            }
        });
    }

    pub fn update(&mut self, _: &UpdateArgs) {
        self.frames_passed += 1;
        if self.frames_passed == self.frames_until_move {
            self.frames_passed = 0;
            if block_intersects(&self, self.block_x as i16, self.block_y as i16 + 1) {
                let (x, y) = (self.block_x, self.block_y);
                imprint_block(self, x as u8, y as u8);

                self.block_x = 2;
                self.block_y = 0;
                self.block = data::from_type(BlockType::rand(&mut rand::StdRng::new().unwrap()));
                if block_intersects(&self, self.block_x, self.block_y) {
                    self.clear();
                }
            }
            else {
                self.block_y += 1;
            }
        }
    }

    pub fn next_rotation(&mut self) {
        self.block_rotation = (self.block_rotation + 1) % self.block.len() as u8;
    }

    pub fn previous_rotation(&mut self) {
        self.block_rotation = if self.block_rotation == 0{
            self.block.len() as u8
        }else{
            self.block_rotation
        } - 1;
    }

    pub fn move_block(&mut self, dx: i16, dy: i16) {
        if !block_intersects(&self, self.block_x + dx, self.block_y + dy) {
            self.block_x += dx;
            self.block_y += dy;
        }
    }

    pub fn clear(&mut self) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.map[i as usize][j as usize] = false;
            }
        }
    }
}
