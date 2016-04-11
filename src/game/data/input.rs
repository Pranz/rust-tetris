#[derive(Copy,Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum Input{
	MoveLeft,
	MoveRight,
	SlowFall,
	FastFall,
	RotateClockwise,
	RotateAntiClockwise,
	Pause,
}

pub mod key{
	use piston::input::Key;
	use std::collections::hash_map::HashMap;

	use ::game::data::Input;
	use ::game::data::PlayerId;

	pub type KeyMap = HashMap<Key,Mapping>;

	#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
	pub struct Mapping{
		pub player          : PlayerId,
		pub input           : Input,
		pub repeat_delay    : f64,//Unit: seconds
		pub repeat_frequency: f64,//Unit: seconds/trigger
	}
}
