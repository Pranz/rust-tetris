use super::super::map::cell::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

pub struct Grid<'ga,'gb,GA: 'ga,GB: 'gb>{
	pub a: &'ga GA,
	pub b: &'gb GB,
	pub b_x: PosAxis,
	pub b_y: PosAxis,
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
    		self.b.pos(self.b_x as usize + x,self.b_y as usize + y)
    	}else{
    		out
    	}
    }
}
