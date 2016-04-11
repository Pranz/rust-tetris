//!A basic tetromino shape (4 blocks)

use ::data::grid::{self,Grid,RectangularBound};
use ::data::Cell;

///All possible tetromino shapes
#[derive(Copy,Clone,Debug,Eq,PartialEq,Rand,Serialize,Deserialize)]
pub enum Shape{
	I,
	L,
	O,
	J,
	T,
	S,
	Z,
}
impl Shape{
	///Number of possible tetromino shapes
	pub const LEN: usize = 7;

	///Returns the data of the tetromino shape
	pub fn data(self,rotation: Rotation) -> (grid::SizeAxis,&'static [bool]){
		let rotation = rotation as usize;
		match self{
			Shape::I => {let &(grid::Size{x,..},ref data) = &data::I;(x,&data[rotation])},
			Shape::L => {let &(grid::Size{x,..},ref data) = &data::L;(x,&data[rotation])},
			Shape::O => {let &(grid::Size{x,..},ref data) = &data::O;(x,&data[rotation])},
			Shape::J => {let &(grid::Size{x,..},ref data) = &data::J;(x,&data[rotation])},
			Shape::T => {let &(grid::Size{x,..},ref data) = &data::T;(x,&data[rotation])},
			Shape::S => {let &(grid::Size{x,..},ref data) = &data::S;(x,&data[rotation])},
			Shape::Z => {let &(grid::Size{x,..},ref data) = &data::Z;(x,&data[rotation])},
		}
	}

	///Returns the number of rotations for the current shape
	fn rotation_count(self) -> Rotation{
		(match self{
			Shape::I => data::I.1.len(),
			Shape::L => data::L.1.len(),
			Shape::O => data::O.1.len(),
			Shape::J => data::J.1.len(),
			Shape::T => data::T.1.len(),
			Shape::S => data::S.1.len(),
			Shape::Z => data::Z.1.len(),
		}) as Rotation
	}

	///Returns the width and height in blocks
	fn size(self) -> grid::Size{
		match self{
			Shape::I => data::I.0,
			Shape::L => data::L.0,
			Shape::O => data::O.0,
			Shape::J => data::J.0,
			Shape::T => data::T.0,
			Shape::S => data::S.0,
			Shape::Z => data::Z.0,
		}
	}

	#[inline(always)]
	pub fn rotations(self) -> ShapeRotations{
		ShapeRotations(RotatedShape{shape: self,rotation: self.rotation_count()})
	}
}

pub type Rotation = u8;

///A shape with its rotation
#[derive(Copy,Clone,Debug,Eq,PartialEq,Rand,Serialize,Deserialize)]
pub struct RotatedShape{
	shape: Shape,
	rotation: Rotation
}

impl RotatedShape{
	#[inline(always)]pub fn new(shape: Shape) -> Self{
		RotatedShape{
			shape   : shape,
			rotation: 0,
		}
	}

	///Returns the current shape rotated 90° anticlockwise
	pub fn rotated_anticlockwise(self) -> Self{RotatedShape{
		rotation: (self.rotation + 1) % self.shape.rotation_count(),
		..self
	}}

	///Returns the current shape rotated 90° clockwise
	pub fn rotated_clockwise(self) -> Self{RotatedShape{
		rotation: if self.rotation == 0{
			self.shape.rotation_count()
		}else{
			self.rotation
		} - 1,
		..self
	}}

	///Number of possible rotations in the range 0 to 360° where the state step is 90° and the rotation state's cells is not equivalent of another rotation state's cells
	///Requirements:
	///    1 <= return_value <= 4
	#[inline(always)]pub fn rotation_count(self) -> Rotation{
		self.shape.rotation_count()
	}

	///Returns the current shape rotated an absolute number of times from the initial rotation with a 90° step
	#[inline(always)]pub fn with_rotation(self,rotation: Rotation) -> Self{RotatedShape{
		rotation: rotation % self.shape.rotation_count(),
		..self
	}}

	///Returns the absolute number of rotations from the current shape's initial position
	#[inline(always)]pub fn rotation(&self) -> Rotation{self.rotation}

	///Returns the shape without rotation
	#[inline(always)]pub fn shape(&self) -> Shape{self.shape}

	///Returns the horizontal center point (x) of the rotated shape
	#[inline(always)]pub fn center_x(&self) -> grid::SizeAxis{
		self.width()/2
	}

	///Returns the vertical center point (y) of the rotated shape
	#[inline(always)]pub fn center_y(&self) -> grid::SizeAxis{
		self.height()/2
	}

	///Returns the center point (x,y) of the rotated shape
	#[inline(always)]pub fn center(&self) -> grid::Size{
		grid::Size{x: self.center_x(),y: self.center_y()}
	}

	//Returns the minimal and maximal horizontal position where that column has a occupied cell
	pub fn real_bound_x(&self) -> Option<(grid::SizeAxis,grid::SizeAxis)>{
		match grid::columns_iter::Iter::new(self).position( |col| grid::column::Iter::new(col).any(|(_,cell)| cell.is_occupied())){
			Some(start) => match grid::columns_iter::Iter::new(self).reversed().rposition(|col| grid::column::Iter::new(col).any(|(_,cell)| cell.is_occupied())){
				Some(end) => Some((
					start as grid::SizeAxis,
					end   as grid::SizeAxis
				)),
				None => None
			},
			None => None
		}
	}
}

impl Grid for RotatedShape{
	type Cell = bool;

	unsafe fn pos(&self, pos: grid::Pos) -> bool{
		let (width,data) = self.shape.data(self.rotation);
		data[pos.x as usize + (pos.y as usize * width as usize)]
	}

	fn is_out_of_bounds(&self,pos: grid::Pos) -> bool{grid::is_position_outside_rectangle(self,pos)}
}

impl grid::RectangularBound for RotatedShape{
	#[inline(always)]fn width(&self) -> grid::SizeAxis{self.shape.size().x}
	#[inline(always)]fn height(&self) -> grid::SizeAxis{self.shape.size().y}
	#[inline(always)]fn size(&self) -> grid::Size{self.shape.size()}
}

///Iterator for every rotation the shape has that isn't equivalent to another in the 360° range with a 90° step
#[derive(Debug)]
pub struct ShapeRotations(RotatedShape);
impl Iterator for ShapeRotations{
	type Item = RotatedShape;

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.0.rotation > 0{
			self.0.rotation-= 1;
			Some(self.0)
		}else{
			None
		}
	}
}

///Contains data arrays of all the possible shapes and its rotations in a 4x4 grid
pub mod data{
	use ::data::grid::Size;

	pub static I: (Size,[[bool; 4*4]; 2]) = (Size{x: 4,y: 4},[
		[
			false, false, true , false,//- - O -
			false, false, true , false,//- - O -
			false, false, true , false,//- - O -
			false, false, true , false,//- - O -
		],[
			false, false, false, false,//- - - -
			false, false, false, false,//- - - -
			true , true , true , true ,//O O O O
			false, false, false, false,//- - - -
		]
	]);

	pub static L: (Size,[[bool; 3*3]; 4]) = (Size{x: 3,y: 3},[
		[
			false, true , false,//- O -
			false, true , false,//- O -
			false, true , true ,//- O O
		],[
			false, false, false,//- - -
			true , true , true ,//O O O
			true , false, false,//O - -
		],[
			true , true , false,//O O -
			false, true , false,//- O -
			false, true , false,//- O -
		],[
			false, false, true ,//- - O
			true , true , true ,//O O O
			false, false, false,//- - -
		]
	]);

	pub static O: (Size,[[bool; 2*2]; 1]) = (Size{x: 2,y: 2},[
		[
			true , true,//O O
			true , true,//O O
		]
	]);

	pub static J: (Size,[[bool; 3*3]; 4]) = (Size{x: 3,y: 3},[
		[
			false, true , false,//- O -
			false, true , false,//- O -
			true , true , false,//O O -
		],[
			true , false, false,//O - -
			true , true , true ,//O O O
			false, false, false,//- - -
		],[
			false, true , true ,//- O O
			false, true , false,//- O -
			false, true , false,//- O -
		],[
			false, false, false,//- - -
			true , true , true ,//O O O
			false, false, true ,//- - O
		]
	]);

	pub static T: (Size,[[bool; 3*3]; 4]) = (Size{x: 3,y: 3},[
		[
			false, false, false,//- - -
			true , true , true ,//O O O
			false, true , false,//- O -
		],[
			false, true , false,//- O -
			true , true , false,//O O -
			false, true , false,//- O -
		],[
			false, true , false,//- O -
			true , true , true ,//O O O
			false, false, false,//- - -
		],[
			false, true , false,//- O -
			false, true , true ,//- O O
			false, true , false,//- O -
		]
	]);

	pub static S: (Size,[[bool; 3*3]; 2]) = (Size{x: 3,y: 3},[
		[
			false, true , false,//- O -
			false, true , true ,//- O O
			false, false, true ,//- - O
		],[
			false, false, false,//- - -
			false, true , true ,//- O O
			true , true , false,//O O -
		]
	]);

	pub static Z: (Size,[[bool; 3*3]; 2]) = (Size{x: 3,y: 3},[
		[
			false, false, true ,//- - O
			false, true , true ,//- O O
			false, true , false,//- O -
		],[
			false, false, false,//- - -
			true , true , false,//O O -
			false, true , true ,//- O O
		]
	]);
}
