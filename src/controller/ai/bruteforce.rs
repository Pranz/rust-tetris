use vec_map::VecMap;
use core::default::Default;
use core::f32;
use core::iter::Iterator;
use piston::input::UpdateArgs;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::grid::{self,translate,Grid};
use data::input::Input;
use data::map::Map;
use data::cell::Cell;
use data::player::Player;
use data::shapes::tetrimino::Shape;
use gamestate;
use game::event::Event;
use tmp_ptr::TmpPtr;

#[derive(Clone)]
pub struct Controller{
	pub input_sender: sync::mpsc::Sender<(Input,gamestate::PlayerId)>,
	pub player_id: gamestate::PlayerId,
	pub settings: Settings,
	move_time_count: f64,
	rotate_time_count: f64,
	target: grid::Pos,
	target_rotation: u8,
}

#[derive(Copy,Clone,Debug,PartialEq)]
pub struct Settings{
	pub move_time: f64,
	pub fall_time: f64,
	pub rotate_time: f64,
}

impl Default for Settings{
	fn default() -> Self{Settings{
		move_time: 0.2,
		fall_time: 0.05,
		rotate_time: 0.4,
	}}
}

impl Controller{
	pub fn new(input_sender: sync::mpsc::Sender<(Input,gamestate::PlayerId)>,player_id: gamestate::PlayerId,settings: Settings) -> Self{Controller{
		input_sender: input_sender,
		player_id: player_id,
		settings: settings,
		move_time_count: 0.0,
		rotate_time_count: 0.0,
		target: grid::Pos{x: 0,y: 0},
		target_rotation: 1,
	}}

	pub fn recalculate_optimal_target<M>(&mut self,map: &M,shape: Shape,pos: grid::Pos)
		where M: Map,
		      <M as grid::Grid>::Cell: Cell + Copy
	{
		let mut greatest_o   = f32::NEG_INFINITY;

		for rotated_shape in shape.rotations(){
			for x in -(rotated_shape.width() as grid::PosAxis)+1 .. map.width() as grid::PosAxis{
				if !grid::is_grid_out_of_bounds(map,&rotated_shape,grid::Pos{x: x,y: 0}){
					let pos = gamestate::fastfallen_shape_pos(
						&rotated_shape,
						map,
						grid::Pos{
							x: x,
							y: pos.y
						}
					);

					let optimality_test_map = grid::imprint_bool::Grid{a: map,b: &translate::Grid{grid: &rotated_shape,pos: grid::Pos{x: -pos.x,y: -pos.y}}};

					let current_o = map_optimality2(&optimality_test_map);
					if current_o > greatest_o{
						greatest_o = current_o;
						self.target = pos;
						self.target_rotation = rotated_shape.rotation();
					}
				}
			}
		}
	}
}

impl<M> ControllerTrait<M,Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::MapId,TmpPtr<M>)>> for Controller
	where M: Map,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	fn update(&mut self,args: &UpdateArgs,players: &VecMap<Player>,_: &VecMap<M>){
		if let Some(player) = players.get(&(self.player_id as usize)){
			self.move_time_count-= args.dt;
			self.rotate_time_count-= args.dt;

			while self.move_time_count <= 0.0{
				if player.pos.x > self.target.x{
					let _ = self.input_sender.send((Input::MoveLeft,self.player_id));
					self.move_time_count+=self.settings.move_time;
				}else if player.pos.x < self.target.x{
					let _ = self.input_sender.send((Input::MoveRight,self.player_id));
					self.move_time_count+=self.settings.move_time;
				}else if player.shape.rotation() == self.target_rotation{
					let _ = self.input_sender.send((Input::SlowFall,self.player_id));
					self.move_time_count+=self.settings.fall_time;
				}else{
					break
				}
			}

			while self.rotate_time_count <= 0.0{
				if player.shape.rotation() != self.target_rotation{
					let _ = self.input_sender.send((Input::RotateAntiClockwise,self.player_id));
					self.rotate_time_count+=self.settings.rotate_time;
				}else{
					break;
				}
			}
		}
	}

	fn event(&mut self,event: Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::MapId,TmpPtr<M>)>){
		use game::event::Event::*;

		match event{
			PlayerAdd{player: (player_id,player),map: (_,map)} if player_id == self.player_id => {
				self.recalculate_optimal_target(&*map,player.shape.shape(),player.pos);
			},
			/*TODO: When other players imprints on the map (Problem: Cannot access self's player)
			MapImprintShape{cause: Some((player_id,_)),map: (_,map),..} if player_id != self.player_id => {
				self.recalculate_optimal_target(&*map,player.shape.shape(),player.pos);
			},*/
			PlayerChangeShape{player: (player_id,_),map: (_,map),shape,pos,..} if player_id == self.player_id => {
				self.move_time_count = 0.0;
				self.rotate_time_count = 0.0;

				self.recalculate_optimal_target(&*map,shape,pos);
			},
			_ => ()
		}
	}
}

#[allow(unused)]
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
#[allow(unused)]
fn map_optimality2<M>(map: &M) -> f32
	where M: grid::Grid,
	      <M as grid::Grid>::Cell: Cell + Copy
{
	let map_height = map.height();
	let rows_completed = grid::rows_iter::Iter::new(map).filter_map(|row| if grid::row::Iter::new(row).all(|(_,cell)| cell.is_occupied()){Some(())}else{None}).count();
	let mut columns_height_sum = 0;
	let mut cells_vertically_blocked_penalty = 0.0;
	let mut height_bumpiness = 0;

	let mut previous_height = None::<grid::SizeAxis>;

	//Iterating columns
	for column in grid::columns_iter::Iter::new(map){
		let mut column = grid::column::Iter::new(column);

		//Find height (First occurence of a occupied cell)
		let height = if let Some((y,_)) = column.find(|&(_,cell)| cell.is_occupied()){
			let height = map_height - y;

			//Count cells vertically blocked
			for (_,cell) in &mut column{
				if cell.is_empty(){
					cells_vertically_blocked_penalty+= height as f32;
				}
			}

			height
		}else{
			0
		};

		columns_height_sum+= height;

		if let Some(previous_height) = previous_height{
			height_bumpiness+= if height > previous_height{height - previous_height}else{previous_height - height}
		}
		previous_height = Some(height);
	}

	(-0.4*columns_height_sum as f32) + (0.25*(rows_completed as f32).powi(2)) + (-0.3*cells_vertically_blocked_penalty as f32) + (-0.3*height_bumpiness.checked_sub(4*2).unwrap_or(0) as f32)
}
