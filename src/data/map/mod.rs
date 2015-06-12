//!A game map

pub mod cell;
pub mod default_map;



use super::shapes::tetrimino::BlockVariant;

///Signed integer type used for describing a position axis. The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;

///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

pub trait Map{
	type Cell;

    //Clears the map
    fn clear(&mut self);

    ///Returns the cell at the given position without checks
    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell;

    ///Sets the cell at the given position without checks
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Self::Cell);

    ///Returns the cell at the given position.
    ///A None will be returned when out of bounds
    fn position(&self,x: PosAxis,y: PosAxis) -> Option<Self::Cell>{
        if x<0 || y<0 || x>=self.width() as PosAxis || y>=self.height() as PosAxis{
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    ///Sets the cell at the given position.
    ///Returns false when out of bounds or failing to set the cell at the given position.
    fn set_position(&mut self,x: PosAxis,y: PosAxis,state: Self::Cell) -> bool{
        if x<0 || y<0 || x>=self.width() as PosAxis || y>=self.height() as PosAxis{
            false
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            true
        }
    }


    ///Collision checks. Whether the given block at the given position will collide with a imprinted block on the map
    fn block_intersects(&self, block: &BlockVariant, x: PosAxis, y: PosAxis) -> Option<(PosAxis, PosAxis)>;

    ///Imprints the given block at the given position on the map
    fn imprint_block<F>(&mut self,block: &BlockVariant, x: PosAxis, y: PosAxis,cell_constructor: F)
        where F: Fn(&BlockVariant) -> Self::Cell;

    ///Check and resolve any full rows, starting to check at the specified y-position and then upward.
    fn handle_full_rows(&mut self, lowest_y: SizeAxis);

    fn width(&self) -> SizeAxis;
    fn height(&self) -> SizeAxis;
}
