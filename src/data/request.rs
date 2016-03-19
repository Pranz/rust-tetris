use ::data::{player,Input};
use ::game;

pub enum Request{
	Input{
		input: Input,
		player: game::data::PlayerId
	},

	PlayerAdd{
		settings: player::Settings
	},

	PlayerRemove{
		player: game::data::PlayerId
	},
}
