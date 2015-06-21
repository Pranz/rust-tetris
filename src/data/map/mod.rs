//!A game map

pub mod cell;
pub mod default_map;
pub mod dynamic_map;



use core::ops::Range;

use super::grid::{self,Grid};
use super::map::cell::Cell;
use super::shapes::tetrimino::ShapeVariant;

///Signed integer type used for describing a position axis. The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;

///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

pub trait Map: Grid{
    ///Sets the cell at the given position.
    ///Returns Err when out of bounds or failing to set the cell at the given position.
    fn set_position(&mut self,x: PosAxis,y: PosAxis,state: Self::Cell) -> Result<(),()>{
        if self.is_position_out_of_bounds(x,y){
            Err(())
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            Ok(())
        }
    }

    ///Sets the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Self::Cell);

    //Clears the map
    fn clear(&mut self) where <Self as Grid>::Cell: cell::Cell{
        for y in 0..self.height(){
            for x in 0..self.width(){
                unsafe{self.set_pos(x as usize,y as usize,Self::Cell::empty())};
            }
        }
    }

    ///Collision checks. Whether the given shape at the given position will collide with a imprinted shape on the map
    fn shape_intersects(&self, shape: &ShapeVariant, x: PosAxis, y: PosAxis) -> CellIntersection;

    ///Imprints the given shape at the given position on the map
    fn imprint_shape<F>(&mut self,shape: &ShapeVariant, x: PosAxis, y: PosAxis,cell_constructor: F)
        where F: Fn(&ShapeVariant) -> Self::Cell//TODO: Probably makes Map not object safe
    {
        for (cell_x,cell_y,cell) in grid::iter::PositionedCellIter::new(shape){
            if cell{
                //TODO: Range checks every iteration
                self.set_position(x+(cell_x as PosAxis),y+(cell_y as PosAxis),cell_constructor(shape)).ok();
            }
        }
    }

    ///Check and resolve any full rows, starting to check at the specified y-position and then upward.
    fn handle_full_rows(&mut self,y: Range<SizeAxis>) -> SizeAxis;

    ///Clears the row at the given y coordinate
    ///Requirements:
    ///    y < height()
    fn clear_row(&mut self,y: SizeAxis);

    ///Copies a row and its cells to another row, overwriting the existing data of the another row
    ///Requirements:
    ///    y_from != y_to
    ///    y_from < height()
    ///    y_to   < height()
    fn copy_row(&mut self,y_from: SizeAxis,y_to: SizeAxis);

    ///Moves a row and its cells to another row, overwriting the existing data of the anotehr row and clears the moved row
    ///Requirements:
    ///    y_from != y_to
    ///    y_from < height()
    ///    y_to   < height()
    fn move_row(&mut self,y_from: SizeAxis,y_to: SizeAxis){
        self.copy_row(y_from,y_to);
        self.clear_row(y_from);
    }
}

pub enum CellIntersection{
    Imprint(PosAxis,PosAxis),
    OutOfBounds(PosAxis,PosAxis),
    None
}

pub mod defaults{
    use super::super::grid::{self,Grid};
    use super::super::shapes::tetrimino::ShapeVariant;
    use super::{Map,PosAxis};
    use super::cell::Cell;

    pub fn shape_intersects<M>(map: &M, shape: &ShapeVariant, x: PosAxis, y: PosAxis) -> super::CellIntersection
        where M: Map,
              <M as Grid>::Cell: Cell + Copy
    {
        for (cell_x,cell_y,cell) in grid::iter::PositionedCellIter::new(shape){
            if cell{
                let (x,y) = (cell_x as PosAxis + x,cell_y as PosAxis + y);
                match map.position(x,y){
                    None                           => return super::CellIntersection::OutOfBounds(x,y),
                    Some(pos) if pos.is_occupied() => return super::CellIntersection::Imprint(x,y),
                    _ => ()
                };
            }
        }
        super::CellIntersection::None
    }
}
