use super::super::cell::Cell;
use super::Grid as GridTrait;
use super::{SizeAxis,Pos};

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

    #[inline]fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        self.grid.is_position_out_of_bounds(Pos{x: self.pos.x + pos.x,y: self.pos.y + pos.y})
    }

    #[inline]fn offset(&self) -> Pos{Pos{
        x: self.pos.x + self.grid.offset().x,
        y: self.pos.y + self.grid.offset().y
    }}
    #[inline]fn width(&self) -> SizeAxis{self.grid.width()}
    #[inline]fn height(&self) -> SizeAxis{self.grid.height()}

    #[inline]unsafe fn pos(&self,pos: Pos) -> Self::Cell{
    	self.grid.pos(Pos{x: self.pos.x + pos.x,y: self.pos.y + pos.y})
    }
}
