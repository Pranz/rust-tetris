//!A game world where player's reside in

pub mod default;
pub mod dynamic;



use core::ops::Range;

use super::grid::{self,Grid,SizeAxis,Pos};
use super::Cell as CellTrait;
use super::shapes::tetromino::RotatedShape;

///Common trait for a World grid used in a game
///A world is always rectangular and all cells within the rectangular boundaries are valid
pub trait World: Grid + grid::RectangularBound{
	///Sets the cell at the given position.
	///Returns Err when out of bounds or failing to set the cell at the given position.
	fn set_position(&mut self,pos: Pos,state: Self::Cell) -> Result<(),()>{
		if self.is_out_of_bounds(pos){
			Err(())
		}else{
			unsafe{self.set_pos(pos.x as usize,pos.y as usize,state)};
			Ok(())
		}
	}

	///Sets the cell at the given position without checks
	///Requirements:
	///    x < height()
	///    y < height()
	unsafe fn set_pos(&mut self,x: usize,y: usize,state: Self::Cell);//TODO: `pos_ref` and `pos_ref_mut` instead

	//Clears the world
	fn clear(&mut self) where <Self as Grid>::Cell: CellTrait{
		for y in 0..self.height(){
			for x in 0..self.width(){
				unsafe{self.set_pos(x as usize,y as usize,Self::Cell::empty())};
			}
		}
	}

	///Collision checks. Whether the given shape at the given position will collide with a imprinted shape on the world
	fn shape_intersects(&self,shape: &RotatedShape,pos: Pos) -> CellIntersection;

	///Imprints the given shape at the given position on the world
	fn imprint_shape(&mut self,shape: &RotatedShape,pos: Pos,cell_constructor: &fn(&RotatedShape) -> Self::Cell){
		for (cell_pos,cell) in grid::cells_iter::Iter::new(shape){
			if cell{
				//TODO: Range checks every iteration
				self.set_position(pos + cell_pos,cell_constructor(shape)).ok();
			}
		}
	}

	///Check and resolve any full rows, starting to check at the specified y-position and then upward.
	fn handle_full_rows(&mut self,y: Range<SizeAxis>) -> SizeAxis;

	///Clears the row at the given y coordinate
	///Requirements:
	///    y < height()
	fn clear_row(&mut self,y: SizeAxis);

	///Copies a row and its cells to another row, overwriting the existing data of the another row
	///Requirements:
	///    y_from != y_to
	///    y_from < height()
	///    y_to   < height()
	fn copy_row(&mut self,y_from: SizeAxis,y_to: SizeAxis);

	///Moves a row and its cells to another row, overwriting the existing data of the anotehr row and clears the moved row
	///Requirements:
	///    y_from != y_to
	///    y_from < height()
	///    y_to   < height()
	fn move_row(&mut self,y_from: SizeAxis,y_to: SizeAxis){
		self.copy_row(y_from,y_to);
		self.clear_row(y_from);
	}
}

///When checking for intersections, these are the different kinds of intersections that can occur
pub enum CellIntersection{
	///Intersects with another imprinted cell in the world
	Imprint(Pos),

	///Intersects with the boundary of the world or the outside non-existent cells
	OutOfBounds(Pos),

	///No intersection
	None
}

///Default methods for a world
pub mod defaults{
	use super::super::grid::{self,Grid,Pos};
	use super::super::shapes::tetromino::RotatedShape;
	use super::super::Cell as CellTrait;
	use super::World;

	pub fn shape_intersects<W>(world: &W,shape: &RotatedShape,pos: Pos) -> super::CellIntersection
		where W: World,
		      <W as Grid>::Cell: CellTrait + Copy
	{
		for (cell_pos,cell) in grid::cells_iter::Iter::new(shape){
			if cell{
				let pos = cell_pos + pos;
				match world.position(pos){
					None                                         => return super::CellIntersection::OutOfBounds(pos),
					Some(world_cell) if world_cell.is_occupied() => return super::CellIntersection::Imprint(pos),
					_ => ()
				};
			}
		}
		super::CellIntersection::None
	}
}
