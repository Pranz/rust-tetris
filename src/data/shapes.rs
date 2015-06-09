use data::gamestate::{GameState,MapPosAxis,MapSizeAxis};
use rand::{Rand,Rng};

pub const BLOCK_SIZE: MapSizeAxis = 4;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BlockType {
    I,
    L,
    O,
    J,
    T,
    S,
    Z,
}

impl BlockType{
    const LEN: usize = 7;

    pub fn data(self) -> &'static [data::Block]{
        match self{
            BlockType::I => &data::I,
            BlockType::L => &data::L,
            BlockType::O => &data::O,
            BlockType::J => &data::J,
            BlockType::T => &data::T,
            BlockType::S => &data::S,
            BlockType::Z => &data::Z,
        }
    }
}


impl Rand for BlockType {
    fn rand<R: Rng>(rng: &mut R) -> Self{
        use self::BlockType::*;

        match rng.gen_range(0,BlockType::LEN as u8){
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
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct BlockVariant {
    pub block_type : BlockType,
    pub rotation   : u8,
}

impl BlockVariant {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        BlockVariant {
            block_type : <BlockType as Rand>::rand(rng),
            rotation   : 0,
        }
    }

    pub fn collision_map(&self) -> &'static [[bool; BLOCK_SIZE as usize]] {
        &self.block_type.data()[self.rotation as usize]
    }

    pub fn next_rotation(&mut self) {
        self.rotation += (self.rotation + 1) % self.block_type.data().len() as u8;
    }

    pub fn previous_rotation(&mut self) {
        self.rotation = if (self.rotation == 0) {
            self.block_type.data().len() as u8
        } else {
            self.rotation
        } - 1;
    }
}

pub mod data {
    pub type Block = [[bool; super::BLOCK_SIZE as usize]; super::BLOCK_SIZE as usize];

    pub static I: [Block; 2] = [
        [
            [false, false, true, false],//O - - -
            [false, false, true, false],//O - - -
            [false, false, true, false],//O - - -
            [false, false, true, false],//O - - -
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
            [false, true , true , false],//O O - -
            [false, true , false, false],//O - - -
            [false, true , false, false],//O - - -
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
            [false, true , false, false],//O - - -
            [false, true , true , false],//O O - -
            [false, true , false, false],//O - - -
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
            [false, false, true , false],//- O - -
            [false, true , true , false],//O O - -
            [false, true , false, false],//O - - -
        ],[
            [false, false, false, false],//- - - -
            [false, false, false, false],//- - - -
            [true , true , false, false],//O O - -
            [false, true , true , false],//- O O -
        ]
    ];
}
