use core::iter;

use super::super::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

///Represents a column in a grid
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<'g,G: 'g>{
	pub grid: &'g G,
	pub x: SizeAxis
}

impl<'g,G> GridTrait for Grid<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Cell = <G as GridTrait>::Cell;

	fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
		if pos.x == self.x as PosAxis + self.offset().x{
			self.grid.is_position_out_of_bounds(pos)
		}else{
			false
		}
	}

	#[inline(always)]fn offset(&self) -> Pos{self.grid.offset()}
	#[inline(always)]fn width(&self) -> SizeAxis{1}
	#[inline(always)]fn height(&self) -> SizeAxis{self.grid.height()}

	#[inline(always)]unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		self.grid.pos(pos)
	}
}

///Iterates through a column's row cells
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,G: 'g>{
	grid: Grid<'g,G>,
	row: SizeAxis
}

impl<'g,G> Iter<'g,G>
	where G: GridTrait + 'g,
{
	#[inline(always)]pub fn new(grid: Grid<'g,G>) -> Self{Iter{grid: grid,row: 0}}
	#[inline(always)]pub fn pos(&self) -> Pos{Pos{x: self.grid.x as PosAxis,y: self.row as PosAxis}}
}

impl<'g,G> iter::Iterator for Iter<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Item = (SizeAxis,<G as GridTrait>::Cell);

	fn next(&mut self) -> Option<Self::Item>{
		if let Some(cell) = self.grid.position(self.pos() + self.grid.offset()){
			let row = self.row;
			self.row+= 1;
			Some((row,cell))
		}else{
			None
		}
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize,Option<usize>){
		let len = self.len();
		(len,Some(len))
	}
}

impl<'g,G> iter::ExactSizeIterator for Iter<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	fn len(&self) -> usize{
		(self.grid.grid.height() - self.row) as usize
	}
}
