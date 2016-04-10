use ::data::{player,Input};
use ::game;

pub enum Request{
	PlayerInput{
		input: Input,
		player: game::data::PlayerId
	},

	PlayerAdd{
		settings: player::Settings,
		world: game::data::WorldId
	},

	PlayerRemove{
		player: game::data::PlayerId
	},

	/*
	PlayerSet{
		player: game::data::PlayerId
		data: Box<Player>
	},

	WorldAdd{
		settings: world::Settings
	},

	WorldRemove{
		world: game::data::WorldId
	}
*/
	WorldRestart{
		world: game::data::WorldId
	}
/*
	WorldPause{
		world: game::data::WorldId
	}

	WorldUnpause{
		world: game::data::WorldId
	}

	WorldSet{
		world: game::data::WorldId
		data: Box<_>
	}

	GameRestart,
	GameQuit,

	NetworkConnect{
		type:,
		address:,
	},
	*/
}
