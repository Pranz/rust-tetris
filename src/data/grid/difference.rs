use core::cmp;

use super::super::Cell;
use super::{Grid as GridTrait,SizeAxis,Pos,PosAxis,RectangularBound};

///This grid only contains the cells where the cell occupation from both differs
pub struct Grid<'ga,'gb,GA: 'ga,GB: 'gb>{
	pub a: &'ga GA,
	pub b: &'gb GB,
}

impl<'ga,'gb,GA,GB> GridTrait for Grid<'ga,'gb,GA,GB>
	where GA: GridTrait + 'ga,
	      GB: GridTrait + 'gb,
	      <GA as GridTrait>::Cell: Cell,
	      <GB as GridTrait>::Cell: Cell
{
	type Cell = bool;

	fn is_out_of_bounds(&self,pos: Pos) -> bool{
		self.a.is_out_of_bounds(pos) ||
		self.b.is_out_of_bounds(pos)
	}

	unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		self.a.pos(pos).is_occupied() && !self.b.pos(pos).is_occupied()
	}

	fn position(&self,pos: Pos) -> Option<Self::Cell>{
		if self.is_out_of_bounds(pos){
			None
		}else{
			Some(unsafe{self.pos(pos)})
		}
	}
}

impl<'ga,'gb,GA,GB> RectangularBound for Grid<'ga,'gb,GA,GB>
	where GA: RectangularBound + 'ga,
	      GB: RectangularBound + 'gb,
{
	fn bound_start(&self) -> Pos{
		let a_offset = self.a.bound_start();
		let b_offset = self.b.bound_start();
		Pos{
			x: cmp::min(a_offset.x,b_offset.x),
			y: cmp::min(a_offset.y,b_offset.y),
		}
	}
	fn width(&self)  -> SizeAxis{
		(cmp::max(self.a.bound_start().x + self.a.width()  as PosAxis,self.b.bound_start().x + self.b.width()  as PosAxis) - self.bound_start().x) as SizeAxis
	}
	fn height(&self) -> SizeAxis{
		(cmp::max(self.a.bound_start().y + self.a.height() as PosAxis,self.b.bound_start().y + self.b.height() as PosAxis) - self.bound_start().y) as SizeAxis
	}
}
