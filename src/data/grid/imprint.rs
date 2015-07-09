use super::super::map::cell::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

///Imprints `b` on `a`
pub struct Grid<'ga,'gb,GA: 'ga,GB: 'gb>{
	pub a: &'ga GA,
	pub b: &'gb GB,
	pub b_pos: Pos,
}

impl<'ga,'gb,GA,GB> GridTrait for Grid<'ga,'gb,GA,GB>
    where GA: GridTrait + 'ga,
          GB: GridTrait<Cell = <GA as GridTrait>::Cell> + 'gb,
          <GA as GridTrait>::Cell: Cell + Copy
{
	type Cell = <GA as GridTrait>::Cell;

    fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        self.a.is_position_out_of_bounds(pos)
    }

    fn width(&self) -> SizeAxis{self.a.width()}
    fn height(&self) -> SizeAxis{self.a.height()}

    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell{
    	let out = self.a.pos(x,y);
    	if out.is_empty(){
            let x = self.b_pos.x as usize + x;
            let y = self.b_pos.y as usize + y;

            if self.b.is_position_out_of_bounds(Pos{x: x as PosAxis,y: y as PosAxis}){
                return self.b.pos(x,y)
            }
    	}
    	
        out
    }
}
