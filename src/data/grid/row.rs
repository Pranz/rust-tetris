use core::iter;

use super::super::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

///Represents a row in a grid
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<'g,G: 'g>{
	pub grid: &'g G,
	pub y: SizeAxis
}

impl<'g,G> GridTrait for Grid<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Cell = <G as GridTrait>::Cell;

	fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
		if pos.y == self.y as PosAxis + self.offset().y{
			self.grid.is_position_out_of_bounds(pos)
		}else{
			false
		}
	}

	#[inline(always)]fn offset(&self) -> Pos{self.grid.offset()}
	#[inline(always)]fn width(&self) -> SizeAxis{self.grid.width()}
	#[inline(always)]fn height(&self) -> SizeAxis{1}

	#[inline(always)]unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		self.grid.pos(pos)
	}
}

///Iterates through a row's column cells
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,G: 'g>{
	grid: Grid<'g,G>,
	column: SizeAxis
}

impl<'g,G> Iter<'g,G>
	where G: GridTrait + 'g,
{
	#[inline(always)]pub fn new(grid: Grid<'g,G>) -> Self{Iter{grid: grid,column: 0}}
	#[inline(always)]pub fn pos(&self) -> Pos{Pos{x: self.column as PosAxis,y: self.grid.y as PosAxis}}
}

impl<'g,G> iter::Iterator for Iter<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Item = (SizeAxis,<G as GridTrait>::Cell);

	fn next(&mut self) -> Option<Self::Item>{
		if let Some(cell) = self.grid.position(self.pos() + self.grid.offset()){
			let column = self.column;
			self.column+= 1;
			Some((column,cell))
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
		(self.grid.grid.width() - self.column) as usize
	}
}
