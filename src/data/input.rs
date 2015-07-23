enum_from_primitive!{
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum Input{
    MoveLeft,
    MoveRight,
    SlowFall,
    FastFall,
    RotateClockwise,
    RotateAntiClockwise,
    Pause,
}}
