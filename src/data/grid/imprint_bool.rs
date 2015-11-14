use core::ops::Deref;

use super::super::Cell;
use super::Grid as GridTrait;
use super::{SizeAxis,Pos,RectangularBound};

///Imprints `b` on `a`
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<GA,GB>{
	pub a: GA,
	pub b: GB,
}

impl<DA,DB,GA,GB> GridTrait for Grid<DA,DB>
	where DA: Deref<Target = GA>,
	      DB: Deref<Target = GB>,
	      GA: GridTrait,
	      GB: GridTrait,
	      <GA as GridTrait>::Cell: Cell + Copy,
	      <GB as GridTrait>::Cell: Cell + Copy,
{
	type Cell = bool;

	#[inline]fn is_out_of_bounds(&self,pos: Pos) -> bool{
		self.a.is_out_of_bounds(pos)
	}

	unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		if self.a.pos(pos).is_occupied(){
			true
		}else{
			if self.b.is_out_of_bounds(pos){
				false
			}else{
				self.b.pos(pos).is_occupied()
			}
		}
	}
}

impl<DA,DB,GA> RectangularBound for Grid<DA,DB>
	where DA: Deref<Target = GA>,
	      GA: RectangularBound,
{
	#[inline]fn bound_start(&self) -> Pos{self.a.bound_start()}
	#[inline]fn width(&self) -> SizeAxis{self.a.width()}
	#[inline]fn height(&self) -> SizeAxis{self.a.height()}
}
