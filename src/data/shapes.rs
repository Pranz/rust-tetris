
use data::gamestate::{GameState, WIDTH, HEIGHT};

pub const BLOCK_SIZE : u8 = 4;
pub type Block = [[bool; BLOCK_SIZE as usize]; BLOCK_SIZE as usize];

pub enum BlockType {
    SquareBlock,
    LBlock,
    LBlockMirrored
}

pub static square_block : [Block; 1] = [[
		[false, false, false, false],
		[false, false, false, false],
		[true , true , false, false],
		[true , true , false, false]
	]];

pub static l_block : [Block; 4] = [
    [
	    [false, true , false, false],
	    [false, true , false, false],
	    [false, true , false, false],
	    [false, true , true , false]
    ],[
        [false, false, false, false],
        [false, false, false, false],
        [true , true , true , true ],
        [true , false, false, false],
    ],[
        [false, true , true , false],
        [false, false, true , false],
        [false, false, true , false],
        [false, false, true , false],
    ],[
        [false, false, false, false],
        [false, false, false, false],
        [true , true , true , true ],
        [false, false, false, true ],
    ]
];

pub static l_block_mirrored : [Block; 4] = [
    [
	    [false, false, true , false],
	    [false, false, true , false],
	    [false, false, true , false],
	    [false, true , true , false]
    ],[
        [false, false, false, false],
        [false, false, false, false],
        [true , false, false, false],
        [true , true , true , true ]
    ],[
        [false, true , true , false],
        [false, true , false, false],
        [false, true , false, false],
        [false, true , false, false]
    ],[
        [false, false, false, false],
        [false, false, false, false],
        [true , true , true , true ],
        [false, false, false, true ]
    ]
];

pub fn block_intersects(gs : &GameState, block : &Block, x : u8, y : u8) -> bool {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block[i as usize][j as usize] {
                if (i+x) >= WIDTH as u8 && (j+y) >= HEIGHT as u8 {
                    return true;
                }
                else if gs.map[(i+x) as usize][(j+y) as usize] {
                    return true;
                }
            }
        }
    }
    false
}

pub fn imprint_block(gs : &mut GameState, block : &Block, x : u8, y : u8) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block[i as usize][j as usize] && (i+x) < WIDTH as u8 && (j+y) < HEIGHT as u8 {
                gs.map[(x+i) as usize][(y+j) as usize] = true;
            }
        }
    }
}
