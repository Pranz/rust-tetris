use core::iter;

use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos,RectangularBound};

///Represents a row in a grid
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<'g,G: 'g>{
	pub grid: &'g G,
	pub y: SizeAxis
}

impl<'g,G> GridTrait for Grid<'g,G>
	where G: GridTrait + RectangularBound + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Cell = <G as GridTrait>::Cell;

	fn is_out_of_bounds(&self,pos: Pos) -> bool{
		if pos.y == self.y as PosAxis + self.bound_start().y{
			self.grid.is_out_of_bounds(pos)
		}else{
			false
		}
	}

	#[inline(always)]unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		self.grid.pos(pos)
	}
}

impl<'g,G> RectangularBound for Grid<'g,G>
	where G: RectangularBound + 'g
{
	#[inline(always)]fn bound_start(&self) -> Pos{self.grid.bound_start()}
	#[inline(always)]fn width(&self) -> SizeAxis{self.grid.width()}
	#[inline(always)]fn height(&self) -> SizeAxis{1}
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
	where G: GridTrait + RectangularBound + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Item = (SizeAxis,<G as GridTrait>::Cell);

	fn next(&mut self) -> Option<Self::Item>{
		if let Some(cell) = self.grid.position(self.pos() + self.grid.bound_start()){
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
	where G: GridTrait + RectangularBound + 'g,
	      <G as GridTrait>::Cell: Copy
{
	fn len(&self) -> usize{
		(self.grid.grid.width() - self.column) as usize
	}
}
