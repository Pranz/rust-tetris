use piston::event;

use super::super::Controller as ControllerTrait;
use data::player::Player;
use data::map::Map;
use data::grid;
use gamestate;

pub struct Controller{
	bounce: bool,
	move_time: f64,
}

impl Controller{
	pub fn new() -> Self{Controller{
		bounce: false,
		move_time: 0.0,
	}}
}

impl<M: Map> ControllerTrait<M> for Controller{
	fn update(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut M){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			if !gamestate::move_player(player,map,grid::Pos{x: if self.bounce{1}else{-1},y: 0}){
				self.bounce = !self.bounce;
			}
			self.move_time -= 0.3;
		}
	}

	fn event(&mut self,_: gamestate::Event,_: &mut Player,_: &mut M){}
}
