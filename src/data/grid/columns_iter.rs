use core::iter;

use super::super::grid::SizeAxis;
use super::{column,Grid};

///Iterates through a grid's columns
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,Grid: 'g>{
	grid: &'g Grid,
	x: SizeAxis,
}

impl<'g,G: Grid> Iter<'g,G>{
	pub fn new(grid: &'g G) -> Self{Iter{grid: grid,x: 0}}
	pub fn reversed(self) -> Self{Iter{x: self.grid.width(),..self}}
}

impl<'g,G> iter::Iterator for Iter<'g,G>
	where G: Grid,
	      G::Cell: Copy
{
	type Item = column::Grid<'g,G>;

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.x < self.grid.width(){
			let x = self.x;
			self.x+=1;
			Some(column::Grid{grid: self.grid,x: x})
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
		self.grid.width() as usize
	}
}

impl<'g,G> iter::DoubleEndedIterator for Iter<'g,G>
	where G: Grid + 'g,
	      <G as Grid>::Cell: Copy
{
	fn next_back(&mut self) -> Option<Self::Item>{
		if self.x == 0{
			None
		}else{
			self.x-=1;
			Some(column::Grid{grid: self.grid,x: self.x})
		}
	}
}
