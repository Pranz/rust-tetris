use core::default::Default;

use super::shapes::tetrimino::{Shape,BlockVariant,BLOCK_COUNT};

pub type PosAxis  = i16;
pub type SizeAxis = u8;
pub type CellType = bool;

pub trait Cell {
	fn is_occupied(self) -> bool;
	fn empty() -> Self;
}

pub struct CellShape(Option<Shape>);

impl Cell for CellShape {
	#[inline(always)]
	fn is_occupied(self) -> bool {self.0.is_some()}
	#[inline(always)]
	fn empty() -> Self {CellShape(None)}
}

impl Cell for bool {
	#[inline(always)]
	fn is_occupied(self) -> bool {self}
	#[inline(always)]
	fn empty() -> Self {false}
}

pub const WIDTH : SizeAxis = 10;
pub const HEIGHT: SizeAxis = 20;

pub struct Map([[CellType; WIDTH as usize]; HEIGHT as usize]);

impl Map{
	pub fn clear(&mut self){
	    for i in 0..WIDTH{
	        for j in 0..HEIGHT{
	            self.set_position(i as PosAxis,j as PosAxis,false);
	        }
	    }
	}

	pub unsafe fn pos(&self,x: usize,y: usize) -> CellType{
	    self.0[y][x]
	}

	pub unsafe fn set_pos(&mut self,x: usize,y: usize,state: CellType){
	    self.0[y][x] = state;
	}

	pub fn position(&self,x: PosAxis,y: PosAxis) -> CellType{
	    if x<0 || y<0 || x>=WIDTH as PosAxis || y>=HEIGHT as PosAxis{
	        Cell::empty()
	    }else{
	        unsafe{self.pos(x as usize,y as usize)}
	    }
	}

	pub fn set_position(&mut self,x: PosAxis,y: PosAxis,state: CellType) -> bool{
	    if x<0 || y<0 || x>=WIDTH as PosAxis || y>=HEIGHT as PosAxis{
	        false
	    }else{
	        unsafe{self.set_pos(x as usize,y as usize,state)};
	        true
	    }
	}

	pub fn block_intersects(&self, block: &BlockVariant, x: PosAxis, y: PosAxis) -> Option<(PosAxis, PosAxis)> {
	    for i in 0..BLOCK_COUNT{
	        for j in 0..BLOCK_COUNT{
	            if block.collision_map()[j as usize][i as usize] {
					let (x, y) = (i as PosAxis + x, j as PosAxis + y);
	                if x < 0 || y < 0 || x >= WIDTH as PosAxis || y >= HEIGHT as PosAxis {
	                    return Some((x,y));
	                }else if unsafe{self.pos(x as usize,y as usize)}{
	                    return Some((x,y));
	                }
	            }
	        }
	    }
	    None
	}

	pub fn imprint_block(&mut self,block: &BlockVariant, x: PosAxis, y: PosAxis){
	    for i in 0 .. BLOCK_COUNT{
	        for j in 0 .. BLOCK_COUNT{
	            if block.collision_map()[j as usize][i as usize]{
	                self.set_position(x+(i as PosAxis),y+(j as PosAxis),true);
	            }
	        }
	    }
	    self.handle_full_rows(y as u8 + 4);//TODO: 4? Magic constant
	}

	//pub fn move_row

	///Check and resolve any full rows, starting to check at the specified y-position and then upward.
	pub fn handle_full_rows(&mut self, lowest_y: SizeAxis){
		let lowest_y = if lowest_y >= HEIGHT{HEIGHT - 1}else{lowest_y};
	    let mut terminated_rows: SizeAxis = 0;
	    for i in 0..BLOCK_COUNT{//TODO: 4? Magic constant
	        let lowest_y = lowest_y - i as SizeAxis + terminated_rows;
	        if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}){
	            terminated_rows += 1;
	            for j in 0..lowest_y{
	                self.0[(lowest_y - j) as usize] = self.0[(lowest_y - j - 1) as usize];
	            }
	            self.0[0] = [false; WIDTH as usize];
	        }
	    }
	}
}

impl Default for Map{
	fn default() -> Self{
		Map([[false; WIDTH as usize]; HEIGHT as usize])
	}
}
