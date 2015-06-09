pub type PosAxis = i16;
pub type SizeAxis = u8;

pub const WIDTH : MapSizeAxis = 10;
pub const HEIGHT: MapSizeAxis = 20;

pub struct Map([[bool; HEIGHT as usize]; WIDTH as usize]);

impl Map{
	pub fn clear(&mut self){
	    for i in 0..WIDTH {
	        for j in 0..HEIGHT {
	            self.set_position(i as MapPosAxis,j as MapPosAxis,false);
	        }
	    }
	}

	pub unsafe fn pos(&self,x: usize,y: usize) -> bool{
	    self.map[x][y]
	}

	pub unsafe fn set_pos(&mut self,x: usize,y: usize,state: bool){
	    self.map[x][y] = state;
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
	    for i in 0..tetrimino::Shape::BLOCK_COUNT {
	        for j in 0..tetrimino::Shape::BLOCK_COUNT {
	            if block[block_rotation as usize][i as usize][j as usize] {
	                if (i as MapPosAxis + x) < 0 || (j as MapPosAxis + y) < 0 || (i as MapPosAxis + x) >= WIDTH as MapPosAxis || (j as MapPosAxis + y) >= HEIGHT as MapPosAxis {
	                    return true;
	                }else if unsafe{self.pos((i as usize) + (x as usize),(j as usize) + (y as usize))}{
	                    return true;
	                }
	            }
	        }
	    }
	    false
	}

	pub fn imprint_block(gs: &mut GameState, x: MapPosAxis, y: MapPosAxis){
	    for i in 0 .. tetrimino::Shape::BLOCK_COUNT{
	        for j in 0 .. tetrimino::Shape::BLOCK_COUNT{
	            if gs.block[gs.block_rotation as usize][i as usize][j as usize]{
	                gs.set_position(x+(i as MapPosAxis),y+(j as MapPosAxis),true);
	            }
	        }
	    }
	}
}
