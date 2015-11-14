use super::Grid as GridTrait;
use super::{SizeAxis,Pos,RectangularBound};

///Imprints `b` on `a`
#[derive(Copy,Clone)]
pub struct Grid<'ga,'gb,GA: GridTrait + 'ga,GB: GridTrait + 'gb,Cell>{
	pub a: &'ga GA,
	pub b: &'gb GB,
	pub map_fn: fn(<GA as GridTrait>::Cell,Option<<GB as GridTrait>::Cell>) -> Cell,
}

impl<'ga,'gb,GA,GB,Cell> GridTrait for Grid<'ga,'gb,GA,GB,Cell>
	where GA: GridTrait + 'ga,
	      GB: GridTrait + 'gb,
	      <GA as GridTrait>::Cell: Copy,
	      <GB as GridTrait>::Cell: Copy,
	      Cell: Copy
{
	type Cell = Cell;

	#[inline]fn is_out_of_bounds(&self,pos: Pos) -> bool{
		self.a.is_out_of_bounds(pos)
	}

	unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		let a_pos = self.a.pos(pos);
		(self.map_fn)(a_pos,if self.b.is_out_of_bounds(pos){Some(self.b.pos(pos))}else{None})
	}
}

impl<'ga,'gb,GA,GB,Cell> RectangularBound for Grid<'ga,'gb,GA,GB,Cell>
	where GA: GridTrait + RectangularBound + 'ga,
	      GB: GridTrait + 'gb
{
	#[inline]fn bound_start(&self) -> Pos{self.a.bound_start()}
	#[inline]fn width(&self) -> SizeAxis{self.a.width()}
	#[inline]fn height(&self) -> SizeAxis{self.a.height()}
}
