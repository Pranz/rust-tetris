
use super::gamestate::{WIDTH, HEIGHT, MapPosAxis, MapSizeAxis, CellType};
use super::shapes::{data, BlockType, BlockVariant, BLOCK_SIZE};

pub fn block_intersects(map: &[[CellType; WIDTH as usize]], block: &BlockVariant, x: MapPosAxis, y: MapPosAxis) -> Option<(MapPosAxis,MapPosAxis)> {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block.collision_map()[i as usize][j as usize] {
                let (x, y) = (i as MapPosAxis + x, j as MapPosAxis + y);
                if x < 0 || y < 0 || x >= WIDTH as MapPosAxis || y >= HEIGHT as MapPosAxis {
                    return Some((x, y));
                }
                else if map[y as usize][x as usize]{
                    return Some((x, y));
                }
            }
        }
    }
    None
}

//check and resolve any full rows, starting to check at the specified y-position and then
//upward.
pub fn handle_full_rows(map: &[[CellType; WIDTH as usize]], lowest_y: MapSizeAxis) {
    let lowest_y = if lowest_y >= HEIGHT { HEIGHT - 1 } else { lowest_y };
    let mut terminated_rows: MapSizeAxis = 0;
    for i in 0..4  {
        let lowest_y = lowest_y - i as MapSizeAxis + terminated_rows;
        if (0..WIDTH).all(|x| map[lowest_y as usize][x as usize]) {
            terminated_rows += 1;
            for j in 0..lowest_y {
                map[(lowest_y - j) as usize] = map[(lowest_y - j - 1) as usize];
            }
            map[0] = [false; WIDTH as usize];
        }
    }
}
pub fn imprint_block(map: &[[CellType; WIDTH as usize]], block: &BlockVariant, x: MapPosAxis, y: MapPosAxis) {
    for i in 0..BLOCK_SIZE {
        for j in 0..BLOCK_SIZE {
            if block.collision_map()[i as usize][j as usize]{
                let (x,y) = (i as MapPosAxis + x, j as MapPosAxis + y);
                if !(x < 0 || y < 0 || x >= WIDTH as MapPosAxis || y >= HEIGHT as MapPosAxis) {
                    map[y as usize][x as usize] = true;
                }
            }
        }
    }
    handle_full_rows(map, y as u8 + 4);
}
