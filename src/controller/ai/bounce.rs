use collections::vec_map::VecMap;
use piston::event;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::input::Input;
use data::player::Player;
use data::map::Map;
use data::grid;
use gamestate;
use game::event::Event;

#[derive(Copy,Clone,PartialEq)]
pub struct Controller{
	player_id: gamestate::PlayerId,
	bounce: bool,
	move_time: f64,
}

impl Controller{
	pub fn new(player_id: gamestate::PlayerId) -> Self{Controller{
		player_id: player_id,
		bounce: false,
		move_time: 0.0,
	}}
}

impl<M: Map,E> ControllerTrait<M,E> for Controller{
	fn update(&mut self,args: &event::UpdateArgs,players: &VecMap<Player>,_: &VecMap<M>){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			/*if !gamestate::move_player(player,map,grid::Pos{x: if self.bounce{1}else{-1},y: 0}){
				self.bounce = !self.bounce;
			}*/
			self.move_time -= 0.3;
		}
	}

	fn event(&mut self,_: E){}
}
