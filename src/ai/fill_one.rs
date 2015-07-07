use piston::event;

use data::player::Player;
use data::map::{self,Map};
use gamestate;

pub struct Ai{
	move_time: f64,
	target_x: map::PosAxis,
	target_y: map::PosAxis,
}

impl Ai{
	pub fn new() -> Self{Ai{
		move_time: 0.0,
		target_x: 0,
		target_y: 0,
	}}

	pub fn update<M: Map>(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut M){
		self.move_time-= args.dt;

		if self.move_time <= 0.0{
			if player.x > self.target_x{
				gamestate::move_player(player,map,-1,0);
				self.move_time+=0.3;
			}else if player.x < self.target_x{
				gamestate::move_player(player,map,1,0);
				self.move_time+=0.3;
			}else{
				gamestate::move_player(player,map,0,1);
				self.move_time+=0.1;
			}
		}
	}

	pub fn event<M: Map>(&mut self,event: gamestate::Event,player: &mut Player,map: &mut M){
		use gamestate::Event::*;

		match event{
			PlayerMoveGravity => (),
			PlayerImprint => (),
			PlayerNewShape => {
				self.target_x = (self.target_x + 1) % (map.width() as map::PosAxis);
			},
		}
	}
}
