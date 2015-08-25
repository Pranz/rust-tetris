pub mod cells_iter;
pub mod column;
pub mod columns_iter;
pub mod difference;
pub mod imprint;
pub mod imprint_bool;
pub mod row;
pub mod rows_iter;
pub mod translate;



use core::ops::{Add,Sub,Neg};

use super::Cell as CellTrait;

///Common trait for a two-dimensional (2D) grid
pub trait Grid{
	type Cell;

	///Returns whether the given position is out of bounds
	fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
		let offset = self.offset();
		pos.x<offset.x || pos.y<offset.y || pos.x>=self.width() as PosAxis+offset.x || pos.y>=self.height() as PosAxis+offset.y
	}

	///Returns the cell at the given position.
	///A None will be returned when out of bounds
	fn position(&self,pos: Pos) -> Option<Self::Cell>{
		if self.is_position_out_of_bounds(pos){
			None
		}else{
			Some(unsafe{self.pos(pos)})
		}
	}

	///Returns the rectangular axis aligned offset of the world
	fn offset(&self) -> Pos{Pos{x: 0,y: 0}}

	///Returns the rectangular axis aligned width of the world
	fn width(&self) -> SizeAxis;

	///Returns the rectangular axis aligned height of the world
	fn height(&self) -> SizeAxis;

	///Returns the rectangular axis aligned size of the world
	fn size(&self) -> Size{
		Size{x: self.width(),y: self.height()}
	}

	///Returns the cell at the given position without checks
	///Requirements:
	///    pos.x < height()
	///    pos.y < height()
	///    is_position_out_of_bounds(pos) == false
	unsafe fn pos(&self,pos: Pos) -> Self::Cell;
}

///Checks whether the `inside`'s occupied cells are inside `outside`
pub fn is_grid_out_of_bounds<GIn,GOut>(outside: &GOut,inside: &GIn,inside_offset: Pos) -> bool
	where GIn : Grid,
	      GOut: Grid,
	      <GIn  as Grid>::Cell: CellTrait + Copy,
{
	for (pos,cell) in cells_iter::Iter::new(inside){
		if cell.is_occupied() && outside.is_position_out_of_bounds(inside_offset + pos){
			return true;
		}
	}
	false
}

///Signed integer type used for describing a position axis.
///The range of `PosAxis` is guaranteed to contain the whole range (also including the negative range) of `SizeAxis`.
pub type PosAxis  = i16;

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
