//!A game map

pub mod cell;



use core::default::Default;

use super::shapes::tetrimino::{BlockVariant,BLOCK_COUNT};

///Signed integer type used for describing a position axis. The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;
///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

///Constant width of the map
pub const WIDTH : SizeAxis = 10;

///Constant height of the map
pub const HEIGHT: SizeAxis = 20;

///Rectangular game map
pub struct Map<Cell>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: cell::Cell + Copy> Map<Cell>{
	//Clears the map
	pub fn clear(&mut self){
	    for i in 0..WIDTH{
	        for j in 0..HEIGHT{
	            self.set_position(i as PosAxis,j as PosAxis,Cell::empty());
	        }
	    }
	}

	///Returns the cell at the given position without checks
	#[inline(always)]
	pub unsafe fn pos(&self,x: usize,y: usize) -> Cell{
	    self.0[y][x]
	}

	///Sets the cell at the given position without checks
	#[inline(always)]
	pub unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
	    self.0[y][x] = state;
	}

	///Returns the cell at the given position.
	///An empty cell will be returned when out of bounds
	pub fn position(&self,x: PosAxis,y: PosAxis) -> Cell{
	    if x<0 || y<0 || x>=WIDTH as PosAxis || y>=HEIGHT as PosAxis{
	        cell::Cell::empty()//TODO: Wouldn't it make sense that everything outside is a occupied cell? The borders are walls after all
	    }else{
	        unsafe{self.pos(x as usize,y as usize)}
	    }
	}

	///Sets the cell at the given position.
	///Returns false when out of bounds or failing to set the cell at the given position.
	pub fn set_position(&mut self,x: PosAxis,y: PosAxis,state: Cell) -> bool{
	    if x<0 || y<0 || x>=WIDTH as PosAxis || y>=HEIGHT as PosAxis{
	        false
	    }else{
	        unsafe{self.set_pos(x as usize,y as usize,state)};
	        true
	    }
	}


	///Collision checks. Whether the given block at the given position will collide with a imprinted block on the map
	pub fn block_intersects(&self, block: &BlockVariant, x: PosAxis, y: PosAxis) -> Option<(PosAxis, PosAxis)> {
	    for i in 0..BLOCK_COUNT{
	        for j in 0..BLOCK_COUNT{
	            if block.collision_map()[j as usize][i as usize] {
					let (x, y) = (i as PosAxis + x, j as PosAxis + y);
	                if x < 0 || y < 0 || x >= WIDTH as PosAxis || y >= HEIGHT as PosAxis {
	                    return Some((x,y));
	                }else if unsafe{self.pos(x as usize,y as usize)}.is_occupied(){
	                    return Some((x,y));
	                }
	            }
	        }
	    }
	    None
	}

	///Imprints the given block at the given position on the map
	pub fn imprint_block<F>(&mut self,block: &BlockVariant, x: PosAxis, y: PosAxis,cell_constructor: F)
		where F: Fn(&BlockVariant) -> Cell
	{
	    for i in 0 .. BLOCK_COUNT{
	        for j in 0 .. BLOCK_COUNT{
	            if block.collision_map()[j as usize][i as usize]{
	                self.set_position(x+(i as PosAxis),y+(j as PosAxis),cell_constructor(block));
	            }
	        }
	    }
	}

	//pub fn move_row

	///Check and resolve any full rows, starting to check at the specified y-position and then upward.
	pub fn handle_full_rows(&mut self, lowest_y: SizeAxis){//TODO: Maybe split the functionality in this function?
		let lowest_y = if lowest_y >= HEIGHT{HEIGHT - 1}else{lowest_y};
	    let mut terminated_rows: SizeAxis = 0;
	    for i in 0..BLOCK_COUNT{
	        let lowest_y = lowest_y - i as SizeAxis + terminated_rows;
	        if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}.is_occupied()){
	            terminated_rows += 1;
	            for j in 0..lowest_y{
	                self.0[(lowest_y - j) as usize] = self.0[(lowest_y - j - 1) as usize];
	            }
	            self.0[0] = [Cell::empty(); WIDTH as usize];
	        }
	    }
	}
}

impl<Cell: cell::Cell + Copy> Default for Map<Cell>{
	fn default() -> Self{
		Map([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
	}
}
