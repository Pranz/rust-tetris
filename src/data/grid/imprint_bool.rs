use super::super::map::cell::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

///Imprints `b` on `a`
pub struct Grid<'ga,'gb,GA: 'ga,GB: 'gb>{
	pub a: &'ga GA,
	pub b: &'gb GB,
	pub b_pos: Pos,
}

impl<'ga,'gb,GA,GB> Grid<'ga,'gb,GA,GB>
    where GA: GridTrait + 'ga,
          GB: GridTrait + 'gb,
          <GA as GridTrait>::Cell: Cell + Copy,
          <GB as GridTrait>::Cell: Cell + Copy,
{
    pub fn is_imprint_outside(&self) -> bool{
        for (pos,cell) in super::iter::PositionedCellIter::new(self.b){
            if cell.is_occupied() && self.a.is_position_out_of_bounds(Pos{x: self.b_pos.x + pos.x as PosAxis,y: self.b_pos.y + pos.y as PosAxis}){
                return true;
            }
        }
        false
    }
}

impl<'ga,'gb,GA,GB> GridTrait for Grid<'ga,'gb,GA,GB>
    where GA: GridTrait + 'ga,
          GB: GridTrait + 'gb,
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
