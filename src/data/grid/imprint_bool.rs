use core::ops::Deref;

use super::super::cell::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

///Imprints `b` on `a`
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Grid<GA,GB>{
	pub a: GA,
	pub b: GB,
	pub b_pos: Pos,
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

    fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        self.a.is_position_out_of_bounds(pos)
    }

    fn width(&self) -> SizeAxis{self.a.width()}
    fn height(&self) -> SizeAxis{self.a.height()}

    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell{
    	if self.a.pos(x,y).is_occupied(){
            true
        }else{
            let x = x as PosAxis - self.b_pos.x;
            let y = y as PosAxis - self.b_pos.y;

            if self.b.is_position_out_of_bounds(Pos{x: x,y: y}){
                false
            }else{
                self.b.pos(x as usize,y as usize).is_occupied()
            }
        }
    }
}
