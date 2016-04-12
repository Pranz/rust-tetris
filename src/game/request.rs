use ::game::data::{player,Input};

///A ingame request sent to perform certain tasks
///Some examples of this in use is when a client sends a request to a server or when some input is being received from a controller
#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Request<P,W>{
	PlayerInput{
		player: P,
		input: Input
	},

	PlayerAdd{
		settings: player::Settings,
		world: W
	},

	PlayerRemove{
		player: P
	},

	/*
	PlayerSet{
		player: P
		data: Box<Player>
	},*/

	/*WorldAdd{
		settings: world::Settings
	},*/

	WorldRemove{
		world: W
	},

	WorldRestart{
		world: W
	},

	WorldPause{
		world: W
	},

	WorldUnpause{
		world: W
	},

	/*WorldSet{
		world: game::data::WorldId
		data: Box<_>
	}*/

	GameRestart,
	GameQuit,
}
