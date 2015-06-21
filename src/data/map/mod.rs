//!A game map

pub mod cell;
pub mod default_map;
pub mod dynamic_map;



use core::ops::Range;

use data::map::cell::Cell;
use super::shapes::tetrimino::ShapeVariant;

///Signed integer type used for describing a position axis. The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;

///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

pub trait Map{
    type Cell;

    //Clears the map
    fn clear(&mut self) where Self::Cell: cell::Cell{
        for y in 0..self.height(){
            for x in 0..self.width(){
                unsafe{self.set_pos(x as usize,y as usize,Self::Cell::empty())};
            }
        }
    }

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

    ///Collision checks. Whether the given shape at the given position will collide with a imprinted shape on the map
    fn shape_intersects(&self, shape: &ShapeVariant, x: PosAxis, y: PosAxis) -> Option<(PosAxis, PosAxis)>;

    ///Imprints the given shape at the given position on the map
    fn imprint_shape<F>(&mut self,shape: &ShapeVariant, x: PosAxis, y: PosAxis,cell_constructor: F)
        where F: Fn(&ShapeVariant) -> Self::Cell//TODO: Probably makes Map not object safe
    {
        for j in 0 .. shape.height(){
            for i in 0 .. shape.width(){
                if shape.pos(i,j){
                    //TODO: Range checks every iteration
                    self.set_position(x+(i as PosAxis),y+(j as PosAxis),cell_constructor(shape)).ok();
                }
            }
        }
    }

    ///Check and resolve any full rows, starting to check at the specified y-position and then upward.
    fn handle_full_rows(&mut self,y: Range<SizeAxis>) -> SizeAxis;

    ///Returns the rectangular axis aligned width of the map
    fn width(&self) -> SizeAxis;

    ///Returns the rectangular axis aligned height of the map
    fn height(&self) -> SizeAxis;

    ///Returns the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell;

    ///Sets the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Self::Cell);

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

pub struct PositionedCellIter<'m,Map: 'm>{
    map: &'m Map,
    x: SizeAxis,
    y: SizeAxis,
}
impl<'m,M: Map> Iterator for PositionedCellIter<'m,M>
    where M: Map,
          M::Cell: Copy
{
    type Item = (SizeAxis,SizeAxis,M::Cell);

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.x == self.map.width(){
            self.x = 0;
            self.y+= 1;
        }

        if self.y == self.map.height(){
            return None
        }

        let x = self.x;
        self.x+=1;

        return Some((x,self.y,unsafe{self.map.pos(x as usize,self.y as usize)}));
    }
}

pub mod defaults{
    use super::super::shapes::tetrimino::ShapeVariant;
    use super::{Map,PosAxis};
    use super::cell::Cell;

    pub fn shape_intersects<M>(map: &M, shape: &ShapeVariant, x: PosAxis, y: PosAxis) -> Option<(PosAxis, PosAxis)>
        where M: Map,
              <M as Map>::Cell: Cell + Copy
    {
        for j in 0..shape.height(){
            for i in 0..shape.width(){
                if shape.pos(i,j){
                    let (x,y) = (i as PosAxis + x,j as PosAxis + y);
                    match map.position(x,y){
                        None                           => return Some((x,y)),
                        Some(pos) if pos.is_occupied() => return Some((x,y)),
                        _ => ()
                    };
                }
            }
        }
        None
    }
}
