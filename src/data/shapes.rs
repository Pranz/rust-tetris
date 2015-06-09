use data::gamestate::{GameState, WIDTH, HEIGHT};
use rand::{Rand,Rng};

pub const BLOCK_SIZE: u8 = 4;

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
impl Rand for BlockType{
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

pub mod data{
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

pub fn block_intersects(gs: &GameState, x: i16, y: i16) -> bool {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if gs.block[gs.block_rotation as usize][i as usize][j as usize] {
                if (i as i16 + x) < 0 || (j as i16 + y) < 0 || (i as i16 + x) >= WIDTH as i16 || (j as i16 + y) >= HEIGHT as i16 {
                    return true;
                }
                else if gs.map[(i as i16 + x) as usize][(j as i16 + y) as usize] {
                    return true;
                }
            }
        }
    }
    false
}

pub fn imprint_block(gs: &mut GameState, x: u8, y: u8) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if gs.block[gs.block_rotation as usize][i as usize][j as usize] && (i+x) < WIDTH as u8 && (j+y) < HEIGHT as u8 {
                gs.map[(x+i) as usize][(y+j) as usize] = true;
            }
        }
    }
}
