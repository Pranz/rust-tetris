enum_from_primitive!{
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
#[repr(u8)]
pub enum Input{
    MoveLeft,
    MoveRight,
    SlowFall,
    FastFall,
    RotateClockwise,
    RotateAntiClockwise,
    Pause,
}}
