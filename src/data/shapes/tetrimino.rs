//!A basic tetrimino shape (4 blocks)

use num::FromPrimitive;
use rand::{Rand,Rng};

use super::super::grid::{self,Grid};

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
    pub fn data(self,rotation: usize) -> (grid::SizeAxis,&'static [bool]){
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
    pub fn rotations(self) -> usize{
        match self{
            Shape::I => data::I.1.len(),
            Shape::L => data::L.1.len(),
            Shape::O => data::O.1.len(),
            Shape::J => data::J.1.len(),
            Shape::T => data::T.1.len(),
            Shape::S => data::S.1.len(),
            Shape::Z => data::Z.1.len(),
        }
    }

    pub fn size(self) -> grid::Size{
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

    pub fn center_x(&self) -> grid::SizeAxis{
        self.width()/2
    }

    pub fn center_y(&self) -> grid::SizeAxis{
        self.height()/2
    }

    pub fn center(&self) -> grid::Size{
        grid::Size{x: self.center_x(),y: self.center_y()}
    }
}

impl Grid for ShapeVariant{
    type Cell = bool;

    unsafe fn pos(&self, x: usize, y: usize) -> bool{
        let (width,data) = self.shape.data(self.rotation as usize);
        data[x + (y * width as usize)]
    }

    #[inline(always)]
    fn width(&self) -> grid::SizeAxis{self.shape.size().x}

    #[inline(always)]
    fn height(&self) -> grid::SizeAxis{self.shape.size().y}

    #[inline(always)]
    fn size(&self) -> grid::Size{self.shape.size()}
}

///Contains data arrays of all the possible shapes and its rotations in a 4x4 grid
pub mod data{
    use super::super::super::grid::Size;

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
