use super::Grid as GridTrait;
use super::{SizeAxis,Pos,RectangularBound};

///Translates `grid`
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<'g,G: 'g>{
	pub grid: &'g G,
	pub pos: Pos,
}

impl<'g,G> GridTrait for Grid<'g,G>
	where G: GridTrait + 'g,
	      <G as GridTrait>::Cell: Copy
{
	type Cell = <G as GridTrait>::Cell;

	#[inline(always)]fn is_out_of_bounds(&self,pos: Pos) -> bool{
		self.grid.is_out_of_bounds(self.pos + pos)
	}

	#[inline(always)]unsafe fn pos(&self,pos: Pos) -> Self::Cell{
		self.grid.pos(self.pos + pos)
	}
}

impl<'g,G> RectangularBound for Grid<'g,G>
	where G: RectangularBound + 'g,
{
	#[inline(always)]fn bound_start(&self) -> Pos{
		self.pos + self.grid.bound_start()
	}
	#[inline(always)]fn width(&self) -> SizeAxis{self.grid.width()}
	#[inline(always)]fn height(&self) -> SizeAxis{self.grid.height()}
}
