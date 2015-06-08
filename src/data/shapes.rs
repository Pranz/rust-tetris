
use data::gamestate::{GameState, WIDTH, HEIGHT};

pub const BLOCK_SIZE : u8 = 4;
pub type Block = [[Bool; BLOCK_SIZE]; BLOCK_SIZE];

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

pub fn block_intersects(gs : &GameState, block : &[Block], x : u8, y : u8) -> bool {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block[i][j] {
                if (i+x) >= WIDTH && (j+y) >= HEIGHT {
                    return true;
                }
                else if gs.map[i+x][j+y] {
                    return true;
                }
            }
        }
    }
    false
}

pub fn imprint_block(gs : &mut GameState, block : &[Block], x : u8, y : u8) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block[i][j] && (i+x) < WIDTH && (j+y) < HEIGHT {
                gs.map[x+i][y+j] = true;
            }
        }
    }
}
