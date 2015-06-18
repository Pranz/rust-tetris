use super::map;
use super::shapes::tetrimino::ShapeVariant;

pub struct Player{
    pub x              : map::PosAxis,
    pub y              : map::PosAxis,
    pub shape          : ShapeVariant,
    pub move_frequency : f64, //Unit: seconds/block
    pub move_time_count: f64, //Unit: seconds
}
