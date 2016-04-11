use ::data::shapes::tetromino::Shape;

///Represents a cell in a grid
pub trait Cell: Sized{
	///Constructs an empty cell
	fn empty() -> Self;

	///Returns whether this cell is non-empty
	fn is_occupied(self) -> bool;

	///Returns whether this cell is empty
	fn is_empty(self) -> bool{!self.is_occupied()}
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
