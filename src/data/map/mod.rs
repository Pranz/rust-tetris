//!A game map

pub mod default_map;
pub mod dynamic_map;



use core::ops::Range;

use super::grid::{self,Grid,PosAxis,SizeAxis,Pos};
use super::cell::Cell as CellTrait;
use super::shapes::tetrimino::ShapeVariant;

pub trait Map: Grid{
    ///Sets the cell at the given position.
    ///Returns Err when out of bounds or failing to set the cell at the given position.
    fn set_position(&mut self,pos: Pos,state: Self::Cell) -> Result<(),()>{
        if self.is_position_out_of_bounds(pos){
            Err(())
        }else{
            unsafe{self.set_pos(pos.x as usize,pos.y as usize,state)};
            Ok(())
        }
    }

    ///Sets the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Self::Cell);

    //Clears the map
    fn clear(&mut self) where <Self as Grid>::Cell: CellTrait{
        for y in 0..self.height(){
            for x in 0..self.width(){
                unsafe{self.set_pos(x as usize,y as usize,Self::Cell::empty())};
            }
        }
    }

    ///Collision checks. Whether the given shape at the given position will collide with a imprinted shape on the map
    fn shape_intersects(&self,shape: &ShapeVariant,pos: Pos) -> CellIntersection;

    ///Imprints the given shape at the given position on the map
    fn imprint_shape(&mut self,shape: &ShapeVariant,pos: Pos,cell_constructor: &fn(&ShapeVariant) -> Self::Cell){
        for (cell_pos,cell) in grid::cells_iter::Iter::new(shape){
            if cell{
                //TODO: Range checks every iteration
                self.set_position(Pos{x: pos.x+(cell_pos.x as PosAxis),y: pos.y+(cell_pos.y as PosAxis)},cell_constructor(shape)).ok();
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
    Imprint(Pos),
    OutOfBounds(Pos),
    None
}

pub mod defaults{
    use super::super::grid::{self,Grid,PosAxis,Pos};
    use super::super::shapes::tetrimino::ShapeVariant;
    use super::super::cell::Cell as CellTrait;
    use super::Map;

    pub fn shape_intersects<M>(map: &M,shape: &ShapeVariant,pos: Pos) -> super::CellIntersection
        where M: Map,
              <M as Grid>::Cell: CellTrait + Copy
    {
        for (cell_pos,cell) in grid::cells_iter::Iter::new(shape){
            if cell{
                let pos = Pos{x: cell_pos.x as PosAxis + pos.x,y: cell_pos.y as PosAxis + pos.y};
                match map.position(pos){
                    None                                     => return super::CellIntersection::OutOfBounds(pos),
                    Some(map_cell) if map_cell.is_occupied() => return super::CellIntersection::Imprint(pos),
                    _ => ()
                };
            }
        }
        super::CellIntersection::None
    }
}
