use opengl_graphics::GlGraphics;
use piston::event::*;
use rand::{self,Rand};

use super::colors;
use super::map::Map;
use super::shapes::{tetrimino,imprint_block};

pub struct GameState {
	map                  : Map,
    pub frames_until_move: u16,
    pub frames_passed    : u16,
    pub block            : &'static [data::Block],
    pub block_rotation   : u8,
    pub block_x          : MapPosAxis,
    pub block_y          : MapPosAxis,
}

impl GameState {
    pub fn new() -> Self {
        let mut state = GameState {
    	    map: [[false; HEIGHT as usize]; WIDTH as usize],
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
                    if self.position(i as MapPosAxis,j as MapPosAxis) {
                    	let transform = c.transform.trans(i as f64 * 16.0, j as f64 * 16.0);
                    	rectangle(colors::WHITE, square, transform, g);
                    }
                }
            }

            for i in 0..tetrimino::Shape::BLOCK_COUNT {
                for j in 0..tetrimino::Shape::BLOCK_COUNT {
                    if self.block[self.block_rotation as usize][i as usize][j as usize] {
                        let transform = c.transform.trans((i as MapPosAxis + self.block_x) as f64 * 16.0, (j as MapPosAxis + self.block_y) as f64 * 16.0);
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
            if self.block_intersects(self.block,self.block_rotation, self.block_x as MapPosAxis, self.block_y as MapPosAxis + 1) {
                let (x, y) = (self.block_x,self.block_y);
                imprint_block(self,x,y);

                self.block = tetrimino::Shape::rand(&mut rand::StdRng::new().unwrap()).data();//TODO: Store StdRng::new
                self.block_x = 2;//TODO: Top middle of map
                self.block_y = 0;
                self.block_rotation = 0;//TODO: Randomize
                if self.block_intersects(self.block,self.block_rotation, self.block_x, self.block_y) {
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

    pub fn move_block(&mut self, dx: MapPosAxis, dy: MapPosAxis) -> bool{
        if self.block_intersects(self.block,self.block_rotation,self.block_x + dx, self.block_y + dy){
            false
        }else{
            self.block_x += dx;
            self.block_y += dy;
            true
        }
    }

    //pub fn move_row
    
    //check and resolve any full rows, starting to check at the specified y-position and then
    //upward.
    pub fn handle_full_rows(&mut self, lowest_y : MapSizeAxis) {
        let mut terminated_rows : MapSizeAxis = 0;
        for i in 0..4  {
            let lowest_y = lowest_y + i as MapSizeAxis - terminated_rows;
            if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}) {
                terminated_rows += 1;
                for j in 0..lowest_y {
                    self.map[(lowest_y - j) as usize] = self.map[(lowest_y - j - 1) as usize];
                }
                self.map[0] = [false; WIDTH as usize];
            }
        }
    }
}
