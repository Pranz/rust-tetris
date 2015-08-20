use core::iter;

use super::super::grid::SizeAxis;
use super::{row,Grid};

///Iterates through a grid's rows
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,Grid: 'g>{
	grid: &'g Grid,
	y: SizeAxis,
}

impl<'g,G: Grid> Iter<'g,G>{
	pub fn new(grid: &'g G) -> Self{Iter{grid: grid,y: 0}}
	pub fn reversed(self) -> Self{Iter{y: self.grid.height(),..self}}
}

impl<'g,G> iter::Iterator for Iter<'g,G>
	where G: Grid,
	      G::Cell: Copy
{
	type Item = row::Grid<'g,G>;

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.y < self.grid.height(){
			let y = self.y;
			self.y+=1;
			Some(row::Grid{grid: self.grid,y: y})
		}else{
			None
		}
	}

	fn size_hint(&self) -> (usize,Option<usize>){
		let len = self.len();
		(len,Some(len))
	}
}

impl<'g,G> iter::ExactSizeIterator for Iter<'g,G>
	where G: Grid + 'g,
	      <G as Grid>::Cell: Copy
{
	fn len(&self) -> usize{
		self.grid.height() as usize
	}
}

impl<'g,G> iter::DoubleEndedIterator for Iter<'g,G>
	where G: Grid + 'g,
	      <G as Grid>::Cell: Copy
{
	fn next_back(&mut self) -> Option<Self::Item>{
		if self.y == 0{
			None
		}else{
			self.y-=1;
			Some(row::Grid{grid: self.grid,y: self.y})
		}
	}
}
