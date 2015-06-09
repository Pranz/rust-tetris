use rand::{Rand,Rng};

use super::super::map;

#[derive(PartialEq, Eq, Copy, Clone)]
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
    pub const LEN: usize = 7;
    pub const BLOCK_COUNT: map::SizeAxis = 4;

    pub fn data(self) -> &'static [data::Block]{
        match self{
            Shape::I => &data::I,
            Shape::L => &data::L,
            Shape::O => &data::O,
            Shape::J => &data::J,
            Shape::T => &data::T,
            Shape::S => &data::S,
            Shape::Z => &data::Z,
        }
    }
}
impl Rand for Shape{
    fn rand<R: Rng>(rng: &mut R) -> Self{
        use self::Shape::*;

        match rng.gen_range(0,Shape::LEN as u8){
            0 => I,
            1 => L,
            2 => O,
            3 => J,
            4 => T,
            5 => S,
            6 => Z,
            _ => unreachable!()
        }
    }
}

pub struct ShapeState{
    shape: Shape,
    rotation: u8
}

pub mod data{
    const BLOCK_COUNT: usize = 4;//TODO: Should be same as super::Shape::BLOCK_COUNT. Bypass rustc panic "Path not fully resolved". May be issue 22933
    pub type Block = [[bool; BLOCK_COUNT as usize]; BLOCK_COUNT as usize];

    pub static I: [Block; 2] = [
        [
            [false, false, true , false],//- - O -
            [false, false, true , false],//- - O -
            [false, false, true , false],//- - O -
            [false, false, true , false],//- - O -
        ],[
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
            [true , true , true , true ],//O O O O
            [false, false, false, false],//- - - -
        ]
    ];

    pub static L: [Block; 4] = [
        [
            [false, true , false, false],//O - - -
            [false, true , false, false],//O - - -
            [false, true , true , false],//O O - -
            [false, false, false, false],//- - - -
        ],[
            [false, false, false, false],//- - - -
            [true , true , true , false],//O O O -
            [true , false, false, false],//O - - -
            [false, false, false, false],//- - - -
        ],[
            [true , true , false, false],//O O - -
            [false, true , false, false],//- O - -
            [false, true , false, false],//- O - -
            [false, false, false, false],//- - - -
        ],[
            [false, false, true , false],//- - O -
            [true , true , true , false],//O O O -
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
        ]
    ];

    pub static O: [Block; 1] = [
        [
            [true , true , false, false],//O O - -
            [true , true , false, false],//O O - -
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
        ]
    ];

    pub static J: [Block; 4] = [
        [
            [false, true , false, false],//- O - -
            [false, true , false, false],//- O - -
            [true , true , false, false],//O O - -
            [false, false, false, false],//- - - -
        ],[
            [true , false, false, false],//O - - -
            [true , true , true , false],//O O O -
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
        ],[
            [false, true , true , false],//- O O -
            [false, true , false, false],//- O - -
            [false, true , false, false],//- O - -
            [false, false, false, false],//- - - -
        ],[
            [false, false, false, false],//- - - -
            [true , true , true , false],//O O O -
            [false, false, true , false],//- - O -
            [false, false, false, false],//- - - -
        ]
    ];

    pub static T: [Block; 4] = [
        [
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
            [true , true , true , false],//O O O -
            [false, true , false, false],//- O - -
        ],[
            [false, false, false, false],//- - - -
            [false, true , false, false],//- O - -
            [true , true , false, false],//O O - -
            [false, true , false, false],//- O - -
        ],[
            [false, false, false, false],//- - - -
            [false, true , false, false],//- O - -
            [true , true , true , false],//O O O -
            [false, false, false, false],//- - - -
        ],[
            [false, false, false, false],//- - - -
            [false, true , false, false],//- O - -
            [false, true , true , false],//- O O -
            [false, true , false, false],//- O - -
        ]
    ];

    pub static S: [Block; 2] = [
        [
            [true , false, false, false],//O - - -
            [true , true , false, false],//O O - -
            [false, true , false, false],//- O - -
            [false, false, false, false],//- - - -
        ],[
            [false, false, false, false],//- - - -
            [false, true , true , false],//- O O -
            [true , true , false, false],//O O - -
            [false, false, false, false],//- - - -
        ]
    ];

    pub static Z: [Block; 2] = [
        [
            [false, false, false, false],//- - - -
            [false, false, true , false],//- - O -
            [false, true , true , false],//- O O -
            [false, true , false, false],//- O - -
        ],[
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
            [true , true , false, false],//O O - -
            [false, true , true , false],//- O O -
        ]
    ];
}
