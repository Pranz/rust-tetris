use piston::event;

use data::grid;
use data::map::Map;
use data::player::Player;
use gamestate;

pub struct Ai{
	move_time: f64,
	target: grid::Pos,
}

impl Ai{
	pub fn new() -> Self{Ai{
		move_time: 0.0,
		target: grid::Pos{x: 0,y: 0},
	}}

	pub fn update<M: Map>(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut M){
		self.move_time-= args.dt;

		if self.move_time <= 0.0{
			if player.pos.x > self.target.x{
				gamestate::move_player(player,map,grid::Pos{x: -1,y: 0});
				self.move_time+=0.3;
			}else if player.pos.x < self.target.x{
				gamestate::move_player(player,map,grid::Pos{x: 1,y: 0});
				self.move_time+=0.3;
			}else{
				gamestate::move_player(player,map,grid::Pos{x: 0,y: 1});
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
				self.target.x = (self.target.x + 1) % (map.width() as grid::PosAxis);
			},
		}
	}
}
