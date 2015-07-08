use piston::event;

use data::player::Player;
use data::map::Map;
use data::grid;
use gamestate;

pub struct Ai{
	bounce: bool,
	move_time: f64,
}

impl Ai{
	pub fn new() -> Self{Ai{
		bounce: false,
		move_time: 0.0,
	}}

	pub fn update<M: Map>(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut M){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			if !gamestate::move_player(player,map,grid::Pos{x: if self.bounce{1}else{-1},y: 0}){
				self.bounce = !self.bounce;
			}
			self.move_time -= 0.3;
		}
	}

	pub fn event<M: Map>(&mut self,_: gamestate::Event,_: &mut Player,_: &mut M){}
}
