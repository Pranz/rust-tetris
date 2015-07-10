pub mod difference;
pub mod imprint;
pub mod imprint_bool;
pub mod iter;

///Signed integer type used for describing a position axis.
///The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;

///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

///Describes a two dimensional position
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Pos {pub x: PosAxis,pub y: PosAxis}

///Describes a two dimensional size
#[derive(Copy,Clone,Eq,PartialEq)]
pub struct Size{pub x: SizeAxis,pub y: SizeAxis}

pub trait Grid{
	type Cell;

    ///Returns whether the given position is out of bounds
    fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        pos.x<0 || pos.y<0 || pos.x>=self.width() as PosAxis || pos.y>=self.height() as PosAxis
    }

    ///Returns the cell at the given position.
    ///A None will be returned when out of bounds
    fn position(&self,pos: Pos) -> Option<Self::Cell>{
        if self.is_position_out_of_bounds(pos){
            None
        }else{
            Some(unsafe{self.pos(pos.x as usize,pos.y as usize)})
        }
    }

    ///Returns the rectangular axis aligned width of the map
    fn width(&self) -> SizeAxis;

    ///Returns the rectangular axis aligned height of the map
    fn height(&self) -> SizeAxis;

    ///Returns the rectangular axis aligned size of the map
    fn size(&self) -> Size{
        Size{x: self.width(),y: self.height()}
    }

    ///Returns the cell at the given position without checks
    ///Requirements:
    ///    x < height()
    ///    y < height()
    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell;
}
