use super::super::shapes::tetrimino::Shape;

pub trait Cell{
    ///Constructs an empty cell
    fn empty() -> Self;

    ///Returns whether this cell is non-empty
    fn is_occupied(self) -> bool;
}

#[derive(Clone,Copy,Eq,PartialEq)]
pub struct ShapeCell(pub Option<Shape>);

impl Cell for ShapeCell{
    #[inline(always)]
    fn empty() -> Self {ShapeCell(None)}

    #[inline(always)]
    fn is_occupied(self) -> bool {self.0.is_some()}
}

impl Cell for bool{
    #[inline(always)]
    fn empty() -> Self {false}

    #[inline(always)]
    fn is_occupied(self) -> bool {self}
}
