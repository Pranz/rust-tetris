use opengl_graphics::GlGraphics;
use piston::event::*;
use rand::{self,Rand};

use super::colors;
use super::shapes::{data,BlockType,BLOCK_SIZE,imprint_block};

pub type MapPosAxis = i16;
pub type MapSizeAxis = u8;

pub const WIDTH : MapSizeAxis = 10;
pub const HEIGHT: MapSizeAxis = 20;

pub struct GameState {
	map                  : [[bool; WIDTH as usize]; HEIGHT as usize],
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
    	    map: [[false; WIDTH as usize]; HEIGHT as usize],
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

            for i in 0..BLOCK_SIZE {
                for j in 0..BLOCK_SIZE {
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

                self.block = BlockType::rand(&mut rand::StdRng::new().unwrap()).data();//TODO: Store StdRng::new
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

    pub fn clear(&mut self){
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_position(i as MapPosAxis,j as MapPosAxis,false);
            }
        }
    }

    pub unsafe fn pos(&self,x: usize,y: usize) -> bool{
        self.map[y][x]
    }

    pub unsafe fn set_pos(&mut self,x: usize,y: usize,state: bool){
        self.map[y][x] = state;
    }

    pub fn position(&self,x: MapPosAxis,y: MapPosAxis) -> bool{
        if x<0 || y<0 || x>=WIDTH as MapPosAxis || y>=HEIGHT as MapPosAxis{
            false
        }else{
            unsafe{self.pos(x as usize,y as usize)}
        }
    }

    pub fn set_position(&mut self,x: MapPosAxis,y: MapPosAxis,state: bool) -> bool{
        if x<0 || y<0 || x>=WIDTH as MapPosAxis || y>=HEIGHT as MapPosAxis{
            false
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            true
        }
    }

    pub fn block_intersects(&self,block: &'static [data::Block],block_rotation: u8, x: MapPosAxis, y: MapPosAxis) -> bool {
        for i in 0..BLOCK_SIZE {
            for j in 0..BLOCK_SIZE {
                if block[block_rotation as usize][i as usize][j as usize] {
                    if (i as MapPosAxis + x) < 0 || (j as MapPosAxis + y) < 0 || (i as MapPosAxis + x) >= WIDTH as MapPosAxis || (j as MapPosAxis + y) >= HEIGHT as MapPosAxis {
                        return true;
                    }else if unsafe{self.pos((i as MapPosAxis + x) as usize,(j as MapPosAxis + y) as usize)}{
                        return true;
                    }
                }
            }
        }
        false
    }
    
    //check and resolve any full rows, starting to check at the specified y-position and then
    //upward.
    pub fn handle_full_rows(&mut self, lowest_y : MapSizeAxis) {
        let lowest_y = if lowest_y >= HEIGHT { HEIGHT - 1 } else { lowest_y };
        let mut terminated_rows : MapSizeAxis = 0;
        for i in 0..4  {
            let lowest_y = lowest_y - i as MapSizeAxis + terminated_rows;
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
