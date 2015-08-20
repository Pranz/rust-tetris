use super::Grid as GridTrait;
use super::{SizeAxis,Pos};

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

	#[inline]fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
		self.a.is_position_out_of_bounds(pos)
	}

	#[inline]fn offset(&self) -> Pos{self.a.offset()}
	#[inline]fn width(&self) -> SizeAxis{self.a.width()}
	#[inline]fn height(&self) -> SizeAxis{self.a.height()}

	unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		let a_pos = self.a.pos(pos);
		(self.map_fn)(a_pos,if self.b.is_position_out_of_bounds(pos){Some(self.b.pos(pos))}else{None})
	}
}
