use super::super::gamestate;
use super::{Input,player};

pub enum Request{
	Input{
		input: Input,
		player: gamestate::PlayerId
	},

	PlayerAdd{
		settings: player::Settings
	},

	PlayerRemove{
		player: gamestate::PlayerId
	},
}
