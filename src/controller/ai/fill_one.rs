use core::default::Default;
use core::f32;
use piston::event;

use super::super::Controller as ControllerTrait;
use data::grid::{self,Grid};
use data::map::Map;
use data::cell::Cell;
use data::player::Player;
use gamestate;

pub struct Controller{
	pub move_time: f64,
	pub fall_time: f64,
	pub rotate_time: f64,
	move_time_count: f64,
	rotate_time_count: f64,
	target: grid::Pos,
	target_rotation: u8,
}

impl Default for Controller{
	fn default() -> Self{Controller{
		move_time: 0.3,
		fall_time: 0.1,
		rotate_time: 0.5,
		move_time_count: 0.0,
		rotate_time_count: 0.0,
		target: grid::Pos{x: 0,y: 0},
		target_rotation: 0,
	}}
}

impl<M> ControllerTrait<M> for Controller
	where M: Map,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	fn update(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut M){
		self.move_time_count-= args.dt;
		self.rotate_time_count-= args.dt;

		if self.move_time_count <= 0.0{
			if player.pos.x > self.target.x{
				gamestate::move_player(player,map,grid::Pos{x: -1,y: 0});
				self.move_time_count+=0.3;
			}else if player.pos.x < self.target.x{
				gamestate::move_player(player,map,grid::Pos{x: 1,y: 0});
				self.move_time_count+=0.3;
			}else if player.shape.rotation() == self.target_rotation{
                player.move_time_count = player.settings.move_frequency;
				gamestate::move_player(player,map,grid::Pos{x: 0,y: 1});
				self.move_time_count+=0.1;
			}
		}

		if self.rotate_time_count <= 0.0{
			if player.shape.rotation() != self.target_rotation{
				player.shape = player.shape.next_rotation();
				self.rotate_time_count+=0.5;
			}
		}
	}

	fn event(&mut self,event: gamestate::Event,player: &mut Player,map: &mut M){
		use gamestate::Event::*;

		match event{
			PlayerMoveGravity => (),
			PlayerImprint => (),
			PlayerNewShape => {
				self.move_time_count = 0.0;
				self.rotate_time_count = 0.0;

				let mut o = f32::INFINITY;

				for rotation in 0 .. player.shape.shape().rotations(){
					for x in -(player.shape.width() as grid::PosAxis)+1 .. map.width() as grid::PosAxis{
						let shape = player.shape.with_rotation(rotation);

						if !grid::is_grid_out_of_bounds(map,&shape,grid::Pos{x: x,y: 0}){
							let optimality_test_map = grid::imprint_bool::Grid{
								a: map,
								b: &shape,
								b_pos: gamestate::fast_fallen_shape(
									&shape,
									map,
									grid::Pos{
										x: x,
										y: player.pos.y
									}
								)
							};

							let o2 = map_optimality(&optimality_test_map);
							if o2 < o{
								o = o2;
								self.target.x = x;
								self.target_rotation = rotation;
							}
						}
					}
				}
			},
		}
	}
}

///Lower is better
fn map_optimality<M>(map: &M) -> f32
	where M: grid::Grid,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	let mut o = 0.0;
	let h = map.height();

	for row in grid::rows_iter::Iter::new(map){
		let y = row.row;
		let penalty = (h - y) as f32 * 20.0;

		for (x,cell) in grid::row::Iter::new(row){
			if cell.is_occupied(){
				o+=(h - y) as f32 * 3.0;
			}else if let Some(cell) = map.position(grid::Pos{x: x as grid::PosAxis,y: y as grid::PosAxis - 1}){
				if cell.is_occupied(){
					o+=penalty;
				}
			}
		}
	}

	o
}
