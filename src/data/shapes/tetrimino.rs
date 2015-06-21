//!A basic tetrimino shape (4 blocks)

use num::FromPrimitive;
use rand::{Rand,Rng};

use super::super::grid::Grid;
use super::super::map;

///All possible tetrimino shapes
enum_from_primitive!{
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Shape{
    I,
    L,
    O,
    J,
    T,
    S,
    Z,
}}
impl Shape{
    ///Number of possible tetrimino shapes
    pub const LEN: usize = 7;

    ///Returns the data of the tetrimino shape
    pub fn data(self,rotation: usize) -> (map::SizeAxis,&'static [bool]){
        match self{
            Shape::I => {let &(w,_,ref data) = &data::I;(w,&data[rotation])},
            Shape::L => {let &(w,_,ref data) = &data::L;(w,&data[rotation])},
            Shape::O => {let &(w,_,ref data) = &data::O;(w,&data[rotation])},
            Shape::J => {let &(w,_,ref data) = &data::J;(w,&data[rotation])},
            Shape::T => {let &(w,_,ref data) = &data::T;(w,&data[rotation])},
            Shape::S => {let &(w,_,ref data) = &data::S;(w,&data[rotation])},
            Shape::Z => {let &(w,_,ref data) = &data::Z;(w,&data[rotation])},
        }
    }

    ///Returns the number of rotations for the current shape
    pub fn rotations(self) -> usize{
        match self{
            Shape::I => data::I.2.len(),
            Shape::L => data::L.2.len(),
            Shape::O => data::O.2.len(),
            Shape::J => data::J.2.len(),
            Shape::T => data::T.2.len(),
            Shape::S => data::S.2.len(),
            Shape::Z => data::Z.2.len(),
        }
    }

    pub fn size(self) -> (map::SizeAxis,map::SizeAxis){
        match self{
            Shape::I => {let (w,h,_) = data::I;(w,h)},
            Shape::L => {let (w,h,_) = data::L;(w,h)},
            Shape::O => {let (w,h,_) = data::O;(w,h)},
            Shape::J => {let (w,h,_) = data::J;(w,h)},
            Shape::T => {let (w,h,_) = data::T;(w,h)},
            Shape::S => {let (w,h,_) = data::S;(w,h)},
            Shape::Z => {let (w,h,_) = data::Z;(w,h)},
        }
    }
}
impl Rand for Shape{
    fn rand<R: Rng>(rng: &mut R) -> Self{
        match Shape::from_u8(rng.gen_range(0,Shape::LEN as u8)){
            Some(out) => out,
            None => unreachable!()
        }
    }
}

///A shape with its rotation
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct ShapeVariant{
    shape: Shape,
    rotation: u8
}

impl ShapeVariant{
    pub fn new(shape: Shape,rotation: u8) -> Self{
        ShapeVariant{
            shape   : shape,
            rotation: rotation,
        }
    }

    pub fn next_rotation(&mut self){
        self.rotation = (self.rotation + 1) % self.shape.rotations() as u8;
    }

    pub fn previous_rotation(&mut self){
        self.rotation = if self.rotation == 0{
            self.shape.rotations() as u8
        }else{
            self.rotation
        } - 1;
    }

    #[inline(always)]
    pub fn shape(&self) -> Shape{self.shape}

    pub fn set_shape(&mut self,shape: Shape){
        self.shape = shape;
        self.rotation %= shape.rotations() as u8;
    }

    /*pub fn random_rotation<R: Rng>(&mut self,rng: &mut R){
        self.rotation = rng.gen_range(0,self.shape.data().len() as u8)
    }*/

    pub fn center_x(&self) -> map::SizeAxis{
        self.width()/2
    }

    pub fn center_y(&self) -> map::SizeAxis{
        self.height()/2
    }

    pub fn center(&self) -> (map::SizeAxis,map::SizeAxis){
        (self.center_x(),self.center_y())
    }
}

impl Grid for ShapeVariant{
    type Cell = bool;

    unsafe fn pos(&self, x: usize, y: usize) -> bool{
        let (width,data) = self.shape.data(self.rotation as usize);
        data[x + (y * width as usize)]
    }

    #[inline(always)]
    fn width(&self) -> map::SizeAxis{self.shape.size().0}

    #[inline(always)]
    fn height(&self) -> map::SizeAxis{self.shape.size().1}
}

///Contains data arrays of all the possible shapes and its rotations in a 4x4 grid
pub mod data{
    use super::super::super::map::SizeAxis;

    pub static I: (SizeAxis,SizeAxis,[[bool; 4*4]; 2]) = (4,4,[
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

    pub static L: (SizeAxis,SizeAxis,[[bool; 3*3]; 4]) = (3,3,[
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

    pub static O: (SizeAxis,SizeAxis,[[bool; 2*2]; 1]) = (2,2,[
        [
            true , true,//O O
            true , true,//O O
        ]
    ]);

    pub static J: (SizeAxis,SizeAxis,[[bool; 3*3]; 4]) = (3,3,[
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

    pub static T: (SizeAxis,SizeAxis,[[bool; 3*3]; 4]) = (3,3,[
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

    pub static S: (SizeAxis,SizeAxis,[[bool; 3*3]; 2]) = (3,3,[
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

    pub static Z: (SizeAxis,SizeAxis,[[bool; 3*3]; 2]) = (3,3,[
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
