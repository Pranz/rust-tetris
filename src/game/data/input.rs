///All the possible inputs a player can input
#[derive(Copy,Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub enum Input{
	///Moves the player to the left
	MoveLeft,

	///Moves the player to the right
	MoveRight,

	///Moves the player down towards the ground
	SlowFall,

	///Moves the player down all the way to the ground
	FastFall,

	///Rotates the player clockwise
	RotateClockwise,

	///Rotates the player anti-clockwise
	RotateAntiClockwise,

	///Pauses the game
	Pause,
}

pub mod key{
	use piston::input::Key;
	use std::collections::hash_map::HashMap;

	use ::game::data::Input;
	use ::game::data::PlayerId;

	///A map that maps a keyboard key to a key mapping, deciding what and how to respond when the key is pressed
	pub type KeyMap = HashMap<Key,Mapping>;

	#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
	pub struct Mapping{
		///Which player the mapping is controlling
		pub player: PlayerId,

		///Which input the mapping should trigger
		pub input: Input,

		///Initial delay after the first key press, before beginning to repeat using the repeat frequency
		///Unit: seconds
		pub repeat_delay: f64,

		///Frequency of how fast the key should be repeating itself after the first repeat delay
		///Unit: seconds/trigger
		pub repeat_frequency: f64,
	}
}
