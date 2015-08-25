use core::ops::Range;

use super::super::grid::{self,Grid};
use super::super::shapes::tetromino::RotatedShape;
use super::super::Cell as CellTrait;
use super::World as WorldTrait;

///Constant width of the world
const WIDTH : grid::SizeAxis = 10;

///Constant height of the world
const HEIGHT: grid::SizeAxis = 20;

///Rectangular static sized game world
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct World<Cell: Copy>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: Copy> Grid for World<Cell>{
	type Cell = Cell;

	#[inline(always)]
	fn width(&self) -> grid::SizeAxis{WIDTH}

	#[inline(always)]
	fn height(&self) -> grid::SizeAxis{HEIGHT}

	#[inline(always)]
	unsafe fn pos(&self,pos: grid::Pos) -> Cell{
		self.0[pos.y as usize][pos.x as usize]
	}
}

impl<Cell: CellTrait + Copy> WorldTrait for World<Cell>{
	#[inline(always)]
	unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
		self.0[y][x] = state;
	}

	fn handle_full_rows(&mut self,y_check: Range<grid::SizeAxis>) -> grid::SizeAxis{
		debug_assert!(y_check.start < y_check.end);
		debug_assert!(y_check.end <= HEIGHT);

		let mut terminated_rows: grid::SizeAxis = 0;

		for y_lowest in y_check.rev(){
			let y_lowest = y_lowest + terminated_rows;
			if (0..WIDTH).all(|x| unsafe{self.pos(grid::Pos{x: x as grid::PosAxis,y: y_lowest as grid::PosAxis})}.is_occupied()){
				terminated_rows += 1;
				for y in (0..y_lowest).rev(){
					self.copy_row(y,y+1);
				}
				self.clear_row(0);
			}
		}

		return terminated_rows;
	}

	#[inline(always)]
	fn clear_row(&mut self,y: grid::SizeAxis){
		debug_assert!(y < self.height());

		self.0[y as usize] = [Cell::empty(); WIDTH as usize];
	}

	#[inline(always)]
	fn copy_row(&mut self,y_from: grid::SizeAxis,y_to: grid::SizeAxis){
		debug_assert!(y_from != y_to);
		debug_assert!(y_from < self.height());
		debug_assert!(y_to < self.height());

		self.0[y_from as usize] = self.0[y_to as usize];
	}

	fn shape_intersects(&self,shape: &RotatedShape,pos: grid::Pos) -> super::CellIntersection{
		super::defaults::shape_intersects(self,shape,pos)
	}
}

impl<Cell: CellTrait + Copy> Default for World<Cell>{
	fn default() -> Self{
		World([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
	}
}