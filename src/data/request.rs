use ::data::{player,Input};

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
