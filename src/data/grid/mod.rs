pub mod cells_iter;
pub mod column;
pub mod columns_iter;
pub mod difference;
pub mod imprint;
pub mod imprint_bool;
pub mod row;
pub mod rows_iter;
pub mod serde;
pub mod translate;



use core::ops::{Add,Sub,Neg};

use super::Cell as CellTrait;

///Common trait for a two-dimensional (2D) grid
pub trait Grid{
	type Cell;

	///Returns whether the given position is out of bounds
	fn is_out_of_bounds(&self,pos: Pos) -> bool;

	///Returns the cell at the given position if it is in bounds.
	///`None` will be returned when out of bounds
	fn position(&self,pos: Pos) -> Option<Self::Cell>{
		if self.is_out_of_bounds(pos){
			None
		}else{
			Some(unsafe{self.pos(pos)})
		}
	}

	///Returns the cell at the given position without checks
	///Requirements:
	///    is_out_of_bounds(pos) == false
	unsafe fn pos(&self,pos: Pos) -> Self::Cell;
}

///Represents a grid bounded by a axis aligned rectangle
///
///When implementing this trait, `width` and `height` needs to be implemented.
///If a different start boundary is needed, then `bound_start` needs to be implemented.
pub trait RectangularBound{
	///Returns a position in the grid where the product of the axes is smallest
	///This means the lower bound (smallest value) of each axis within the grid
	///
	///Requirements:
	///  bound_end.x >= bound_start.x
	///  bound_end.y >= bound_start.y
	///  size = bound_end - bound_start
	#[inline(always)]
	fn bound_start(&self) -> Pos{Pos{x: 0,y: 0}}

	///Returns a position in the grid where the product of the axes is greatest
	///This means the upper bound (greatest value) of each axis within the grid
	///
	///Requirements:
	///  bound_end.x >= bound_start.x
	///  bound_end.y >= bound_start.y
	///  size == bound_end - bound_start - Size(1,1)
	#[inline(always)]
	fn bound_end(&self) -> Pos{self.bound_start() + self.size() - Size{x: 1,y: 1}}

	///Returns the width of the boundary
	///
	///Requirements:
	///  size.x == width
	fn width(&self) -> SizeAxis;

	///Returns the height of the boundary
	///
	///Requirements:
	///  size.y == height
	fn height(&self) -> SizeAxis;

	///Returns the size (size of each axis) of the boundary
	///
	///Requirements:
	///  size == bound_end - bound_start
	///  size.x == width
	///  size.y == height
	#[inline(always)]
	fn size(&self) -> Size{
		Size{x: self.width(),y: self.height()}
	}

	///Returns the rectangular axis aligned bounds of the world
	///
	///Requirements:
	///  bounds = (bound_start,bound_end)
	#[inline(always)]
	fn bounds(&self) -> (Pos,Pos){(self.bound_start(),self.bound_end())}

	unsafe fn pos_relative(&self,pos: Pos) -> <Self as Grid>::Cell
		where Self: Grid
	{
		self.pos(pos + self.bound_start())
	}
}

///Returns whether the given position is outside the rectangle
pub fn is_position_outside_rectangle<R>(r: &R,pos: Pos) -> bool
	where R: RectangularBound
{
	let (Pos{x: left,y: top},Pos{x: right,y: bottom}) = r.bounds();
	pos.x<left || pos.y<top || pos.x>right || pos.y>bottom
}

///Returns whether the first rectangle is outside the second rectangle
pub fn is_rectangle_outside_rectangle<R1,R2>(r1: R1,r2: R2) -> bool
	where R1: RectangularBound,
	      R2: RectangularBound
{
	let (Pos{x: left1,y: top1},Pos{x: right1,y: bottom1}) = r1.bounds();
	let (Pos{x: left2,y: top2},Pos{x: right2,y: bottom2}) = r2.bounds();
	right1 <= left2 || left1 >= right2 || bottom1 <= top2 || top1 >= bottom2
}

///Checks whether the `inside`'s occupied cells are inside `outside`
pub fn is_grid_out_of_bounds<GIn,GOut>(outside: &GOut,inside: &GIn) -> bool
	where GIn : Grid + RectangularBound,
	      GOut: Grid,
	      <GIn  as Grid>::Cell: CellTrait + Copy,
{
	for (pos,cell) in cells_iter::Iter::new(inside){
		if cell.is_occupied() && !outside.is_out_of_bounds(inside.bound_start() + pos){
			return false;
		}
	}
	true
}

///Signed integer type used for describing a position axis.
///The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis = i16;

///Unsigned integer type used for describing a size axis.
pub type SizeAxis = u8;

///Describes a two dimensional position
#[derive(Copy,Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
#[repr(simd)]//TODO: Maybe this will be useful in the future?
pub struct Pos{pub x: PosAxis,pub y: PosAxis}

///Describes a two dimensional size
#[derive(Copy,Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
#[repr(simd)]//TODO: Maybe this will be useful in the future?
pub struct Size{pub x: SizeAxis,pub y: SizeAxis}

///Provides specialization for `Pos::with_*` and `Size::with_*`.
///Those methods is able to do that with the help of `_Map` without imports when used in other modules, bypassing the strict impl rules
pub trait _Map<T>{fn apply(self,value: T) -> T;}

impl Add for Pos{
	type Output = Self;
	#[inline(always)]fn add(self,other: Self) -> Self{Pos{x: self.x+other.x,y: self.y+other.y}}
}
impl Sub for Pos{
	type Output = Self;
	#[inline(always)]fn sub(self,other: Self) -> Self{Pos{x: self.x-other.x,y: self.y-other.y}}
}
impl Neg for Pos{
	type Output = Self;
	#[inline(always)]fn neg(self) -> Self{Pos{x: -self.x,y: -self.y}}
}
impl Add<Size> for Pos{
	type Output = Self;
	#[inline(always)]fn add(self,other: Size) -> Self{Pos{x: self.x+other.x as PosAxis,y: self.y+other.y as PosAxis}}
}
impl Sub<Size> for Pos{
	type Output = Self;
	#[inline(always)]fn sub(self,other: Size) -> Self{Pos{x: self.x-other.x as PosAxis,y: self.y-other.y as PosAxis}}
}
impl _Map<PosAxis> for PosAxis{
	#[inline(always)]fn apply(self,_: PosAxis) -> PosAxis{self}
}
impl<F,A> _Map<PosAxis> for F where F: FnOnce(PosAxis) -> A,A: _Map<PosAxis>{
	#[inline(always)]fn apply(self,value: PosAxis) -> PosAxis{A::apply(self(value),value)}
}
impl _Map<PosAxis> for SizeAxis{
	#[inline(always)]fn apply(self,_: PosAxis) -> PosAxis{self as PosAxis}
}
impl Pos{
	#[inline(always)]pub fn with_x<V: _Map<PosAxis>>(self,x: V) -> Self{Pos{x: x.apply(self.x),y: self.y}}
	#[inline(always)]pub fn with_y<V: _Map<PosAxis>>(self,y: V) -> Self{Pos{x: self.x,y: y.apply(self.y)}}
	#[inline(always)]pub fn and_then_x<V: _Map<PosAxis>>(&mut self,map: V) -> Self{
		let x = self.x;
		self.x = map.apply(self.x);
		self.with_x(x)
	}
	#[inline(always)]pub fn and_then_y<V: _Map<PosAxis>>(&mut self,map: V) -> Self{
		let y = self.y;
		self.y = map.apply(self.y);
		self.with_x(y)
	}
	#[inline(always)]pub fn and_then<VX: _Map<PosAxis>,VY: _Map<PosAxis>>(&mut self,map_x: VX,map_y: VY) -> Self{
		let copy = *self;
		self.x = map_x.apply(self.x);
		self.y = map_y.apply(self.y);
		copy
	}
}

impl Add for Size{
	type Output = Self;
	#[inline(always)]fn add(self,other: Self) -> Self{Size{x: self.x+other.x,y: self.y+other.y}}
}
impl Sub for Size{
	type Output = Self;
	#[inline(always)]fn sub(self,other: Self) -> Self{Size{x: self.x-other.x,y: self.y-other.y}}
}
impl Add<Pos> for Size{
	type Output = Pos;
	#[inline(always)]fn add(self,other: Pos) -> Pos{Pos::add(other,self)}
}
impl _Map<SizeAxis> for SizeAxis{
	#[inline(always)]fn apply(self,_: SizeAxis) -> SizeAxis{self}
}
impl<F,A> _Map<SizeAxis> for F where F: FnOnce(SizeAxis) -> A,A: _Map<SizeAxis>{
	#[inline(always)]fn apply(self,value: SizeAxis) -> SizeAxis{A::apply(self(value),value)}
}
impl Size{
	#[inline(always)]pub fn with_x<V: _Map<SizeAxis>>(self,x: V) -> Self{Size{x: x.apply(self.x),y: self.y}}
	#[inline(always)]pub fn with_y<V: _Map<SizeAxis>>(self,y: V) -> Self{Size{x: self.x,y: y.apply(self.y)}}
	#[inline(always)]pub fn and_then_x<V: _Map<SizeAxis>>(&mut self,map: V) -> Self{
		let x = self.x;
		self.x = map.apply(self.x);
		self.with_x(x)
	}
	#[inline(always)]pub fn and_then_y<V: _Map<SizeAxis>>(&mut self,map: V) -> Self{
		let y = self.y;
		self.y = map.apply(self.y);
		self.with_x(y)
	}
	#[inline(always)]pub fn and_then<VX: _Map<SizeAxis>,VY: _Map<SizeAxis>>(&mut self,map_x: VX,map_y: VY) -> Self{
		let copy = *self;
		self.x = map_x.apply(self.x);
		self.y = map_y.apply(self.y);
		copy
	}
}
