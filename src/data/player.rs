use super::map::{self,Map};
use super::shapes::tetrimino::{Shape,BlockVariant};

pub struct Player {
    pub x             : map::PosAxis,
    pub y             : map::PosAxis,
    pub block         : BlockVariant,
    pub move_frequency: f64, //Unit: seconds/block
    pub map_id        : u8,
}

impl Player {

}
