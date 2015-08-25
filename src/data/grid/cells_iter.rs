use super::super::grid::Size;
use super::Grid;

///Iterates through a grid's cells
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,Grid: 'g>{
	grid: &'g Grid,
	pos: Size,
}

impl<'g,G: Grid> Iter<'g,G>{
	pub fn new(grid: &'g G) -> Self{Iter{grid: grid,pos: Size{x: 0,y: 0}}}
}

impl<'g,G: Grid> Iterator for Iter<'g,G>
	where G: Grid,
	      G::Cell: Copy
{
	type Item = (Size,G::Cell);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		loop{
			if self.pos.x == self.grid.width(){
				self.pos.x = 0;
				self.pos.y+= 1;
			}

			if self.pos.y == self.grid.height(){
				return None
			}

			let x = self.pos.x;
			self.pos.x+=1;

			match self.grid.position(self.pos.with_x(x) + self.grid.offset()){
				Some(cell) => return Some((Size{x: x,y: self.pos.y},cell)),
				None => continue
			}
		}
	}
}
