pub mod iter;

use super::map::{PosAxis,SizeAxis};

pub trait Grid{
	type Cell;

    ///Returns whether the given position is out of bounds
    fn is_position_out_of_bounds(&self,x: PosAxis,y: PosAxis) -> bool{
        x<0 || y<0 || x>=self.width() as PosAxis || y>=self.height() as PosAxis
    }

    ///Returns the cell at the given position.
    ///A None will be returned when out of bounds
    fn position(&self,x: PosAxis,y: PosAxis) -> Option<Self::Cell>{
        if self.is_position_out_of_bounds(x,y){
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    ///Returns the rectangular axis aligned width of the map
    fn width(&self) -> SizeAxis;

    ///Returns the rectangular axis aligned height of the map
    fn height(&self) -> SizeAxis;

    ///Returns the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell;
}
