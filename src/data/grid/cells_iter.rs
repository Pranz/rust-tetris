use super::{Grid,Pos,RectangularBound};

///Iterates through a grid's cells
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Iter<'g,Grid: 'g>{
	grid: &'g Grid,
	pos: Pos,
}

impl<'g,G> Iter<'g,G>
	where G: Grid + RectangularBound
{
	#[inline(always)]pub fn new(grid: &'g G) -> Self{Iter{grid: grid,pos: grid.bound_start()}}
	#[inline(always)]pub fn i(self) -> Pos{self.pos}
}

impl<'g,G> Iterator for Iter<'g,G>
	where G: Grid + RectangularBound,
	      G::Cell: Copy
{
	type Item = (Pos,G::Cell);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		let bound_end = self.grid.bound_end();
		loop{
			if self.pos.x > bound_end.x{
				self.pos.x = self.grid.bound_start().x;
				self.pos.y+= 1;
			}

			if self.pos.y > bound_end.y{
				return None
			}

			let x = self.pos.x;
			self.pos.x+=1;

			let old_pos = self.pos.with_x(x);
			match self.grid.position(old_pos){
				Some(cell) => return Some((old_pos,cell)),
				None       => continue
			}
		}
	}
}
