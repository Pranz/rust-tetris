use core::default::Default;
use core::f32;
use core::iter::Iterator;
use piston::event;

use super::super::Controller as ControllerTrait;
use data::grid::{self,Grid};
use data::map::Map;
use data::cell::Cell;
use data::player::Player;
use gamestate;

#[derive(Copy,Clone,PartialEq)]
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

		while self.move_time_count <= 0.0{
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
			}else{
				break
			}
		}

		while self.rotate_time_count <= 0.0{
			if player.shape.rotation() != self.target_rotation{
				player.shape = player.shape.rotated_anticlockwise();
				self.rotate_time_count+=0.5;
			}else{
				break;
			}
		}
	}

	fn event(&mut self,event: gamestate::Event,player: &mut Player,map: &mut M){
		use gamestate::Event::*;

		match event{
			PlayerMoveGravity => (),
			PlayerImprint => (),
			PlayerRowsClear{..} => (),
			PlayerNewShape{new: new_shape,..} => {
				self.move_time_count = 0.0;
				self.rotate_time_count = 0.0;

				let mut o = f32::NEG_INFINITY;

				for shape in new_shape.rotations(){
					for x in -(shape.width() as grid::PosAxis)+1 .. map.width() as grid::PosAxis{
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

							let o2 = map_optimality2(&optimality_test_map);
							if o2 > o{
								o = o2;
								self.target.x = x;
								self.target_rotation = shape.rotation();
							}
						}
					}
				}
			},
		}
	}
}

fn map_optimality<M>(map: &M) -> f32
	where M: grid::Grid,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	let mut o = 0.0;
	let map_height = map.height();

	for row in grid::rows_iter::Iter::new(map){
		let y = row.y;
		let height = map_height - y;
		let penalty = height as f32 * 20.0;

		for (x,cell) in grid::row::Iter::new(row){
			if cell.is_occupied(){
				o+= height as f32 * 3.0;
			}else if let Some(cell) = map.position(grid::Pos{x: x as grid::PosAxis,y: y as grid::PosAxis - 1}){
				if cell.is_occupied(){
					o+= penalty;
				}
			}
		}
	}

	-o
}

///Greater is better
fn map_optimality2<M>(map: &M) -> f32
	where M: grid::Grid,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	let map_height = map.height();
	let rows_completed = grid::rows_iter::Iter::new(map).filter_map(|row| if grid::row::Iter::new(row).all(|(_,cell)| cell.is_occupied()){Some(())}else{None}).count();
	let mut columns_height_sum       = 0;
	let mut cells_vertically_blocked = 0;
	let mut height_bumpiness         = 0;

	let mut previous_height = None::<grid::SizeAxis>;

	//Iterating columns
	for column in grid::columns_iter::Iter::new(map){
		let mut column = grid::column::Iter::new(column);

		//Find height (First occurence of a occupied cell)
		let height = if let Some((y,_)) = column.find(|&(_,cell)| cell.is_occupied()){
			//Count cells vertically blocked
			for (_,cell) in &mut column{
				if cell.is_empty(){
					cells_vertically_blocked+= 1;
				}
			}

			map_height - y
		}else{
			0
		};

		columns_height_sum+= height;

		if let Some(previous_height) = previous_height{
			height_bumpiness+= if height > previous_height{height - previous_height}else{previous_height - height}
		}
		previous_height = Some(height);
	}

	(-0.5*columns_height_sum as f32) + (1.0*rows_completed as f32) + (-0.1*cells_vertically_blocked as f32) + (-0.2*height_bumpiness.checked_sub(4).unwrap_or(0) as f32)
}
