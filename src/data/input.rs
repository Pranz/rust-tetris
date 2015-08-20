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

pub mod key{
    use piston::input::Key;
    use std::collections::hash_map::HashMap;

    use data::input::Input;
    use gamestate::PlayerId;

    pub type KeyMap = HashMap<Key,Mapping>;

    #[derive(Copy,Clone,PartialEq)]
    pub struct Mapping{
        pub player          : PlayerId,
        pub input           : Input,
        pub repeat_delay    : f64,//Unit: seconds
        pub repeat_frequency: f64,//Unit: seconds/trigger
    }
}
