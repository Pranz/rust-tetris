use ::data::{player,Input};
use ::game;

#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Player<P>{
	Input{
		player: P,
		input: Input
	},

	Add{
		settings: player::Settings,
		world: game::data::WorldId
	},

	Remove{
		player: P
	},

	/*
	Set{
		player: P
		data: Box<Player>
	},*/
}

#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum World<W>{
	/*Add{
		settings: world::Settings
	},*/

	Remove{
		world: W
	},

	Restart{
		world: W
	},

	Pause{
		world: W
	},

	Unpause{
		world: W
	},

	/*Set{
		world: game::data::WorldId
		data: Box<_>
	}*/
}

#[derive(Copy,Clone,Debug,PartialEq,Serialize,Deserialize)]
pub enum Request<P,W>{//TODO: Merge this somehow with the client packets in online multiplayer
	Player(Player<P>),
	World(World<W>),

	GameRestart,
	GameQuit,
}
