use core::cmp;

use super::super::cell::Cell;
use super::Grid as GridTrait;
use super::{SizeAxis,Pos};

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

    fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        self.a.is_position_out_of_bounds(pos) ||
        self.b.is_position_out_of_bounds(pos)
    }

    fn width(&self)  -> SizeAxis{cmp::min(self.a.width() ,self.b.width())}
    fn height(&self) -> SizeAxis{cmp::min(self.a.height(),self.b.height())}

    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell{
        self.a.pos(x,y).is_occupied() && !self.b.pos(x,y).is_occupied()
    }

    fn position(&self,pos: Pos) -> Option<Self::Cell>{
        if self.is_position_out_of_bounds(pos){
            None
        }else{
            Some(unsafe{self.pos(pos.x as usize,pos.y as usize)})
        }
    }
}
