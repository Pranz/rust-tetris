use super::super::shapes::tetrimino::Shape;

pub trait Cell{
    ///Constructs an empty cell
    fn empty() -> Self;

    ///Returns whether this cell is non-empty
    fn is_occupied(self) -> bool;
}

pub struct ShapeCell(Option<Shape>);

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

impl Cell for u8 {
    #[inline(always)]
    fn empty() -> Self {0}
    #[inline(always)]
    fn is_occupied(self) -> bool {self > 0}
}
