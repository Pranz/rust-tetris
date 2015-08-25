use vec_map::VecMap;
use core::default::Default;
use core::f32;
use core::iter::Iterator;
use piston::input::UpdateArgs;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::grid::{self,translate};
use data::shapes::tetromino::Shape;
use data::{Cell,Input,Grid,Player,Request,World};
use game::Event;
use gamestate;
use tmp_ptr::TmpPtr;

#[derive(Clone)]
pub struct Controller{
	pub request_sender: sync::mpsc::Sender<Request>,
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
	pub fn new(request_sender: sync::mpsc::Sender<Request>,player_id: gamestate::PlayerId,settings: Settings) -> Self{Controller{
		request_sender: request_sender,
		player_id: player_id,
		settings: settings,
		move_time_count: 0.0,
		rotate_time_count: 0.0,
		target: grid::Pos{x: 0,y: 0},
		target_rotation: 1,
	}}

	pub fn recalculate_optimal_target<W>(&mut self,world: &W,shape: Shape,pos: grid::Pos)
		where W: World,
		      <W as Grid>::Cell: Cell + Copy
	{
		let mut greatest_o   = f32::NEG_INFINITY;

		for rotated_shape in shape.rotations(){
			for x in -(rotated_shape.width() as grid::PosAxis)+1 .. world.width() as grid::PosAxis{
				if !grid::is_grid_out_of_bounds(world,&rotated_shape,grid::Pos{x: x,y: 0}){
					let pos = gamestate::fastfallen_shape_pos(
						&rotated_shape,
						world,
						pos.with_x(x)
					);

					let optimality_test_world = grid::imprint_bool::Grid{a: world,b: &translate::Grid{grid: &rotated_shape,pos: -pos}};

					let current_o = world_optimality2(&optimality_test_world);
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

impl<W> ControllerTrait<W,Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::WorldId,TmpPtr<W>)>> for Controller
	where W: World,
	      <W as Grid>::Cell: Cell + Copy
{
	fn update(&mut self,args: &UpdateArgs,players: &VecMap<Player>,_: &VecMap<W>){
		if let Some(player) = players.get(&(self.player_id as usize)){
			self.move_time_count-= args.dt;
			self.rotate_time_count-= args.dt;

			while self.move_time_count <= 0.0{
				if player.pos.x > self.target.x{
					let _ = self.request_sender.send(Request::Input{input: Input::MoveLeft,player: self.player_id});
					self.move_time_count+=self.settings.move_time;
				}else if player.pos.x < self.target.x{
					let _ = self.request_sender.send(Request::Input{input: Input::MoveRight,player: self.player_id});
					self.move_time_count+=self.settings.move_time;
				}else if player.shape.rotation() == self.target_rotation{
					let _ = self.request_sender.send(Request::Input{input: Input::SlowFall,player: self.player_id});
					self.move_time_count+=self.settings.fall_time;
				}else{
					break
				}
			}

			while self.rotate_time_count <= 0.0{
				if player.shape.rotation() != self.target_rotation{
					let _ = self.request_sender.send(Request::Input{input: Input::RotateAntiClockwise,player: self.player_id});
					self.rotate_time_count+=self.settings.rotate_time;
				}else{
					break;
				}
			}
		}
	}

	fn event(&mut self,event: Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::WorldId,TmpPtr<W>)>){
		use game::Event::*;

		match event{
			PlayerAdd{player: (player_id,player),world: (_,world)} if player_id == self.player_id => {
				self.recalculate_optimal_target(&*world,player.shape.shape(),player.pos);
			},
			/*TODO: When other players imprints on the world (Problem: Cannot access self's player)
			WorldImprintShape{cause: Some((player_id,_)),world: (_,world),..} if player_id != self.player_id => {
				self.recalculate_optimal_target(&*world,player.shape.shape(),player.pos);
			},*/
			PlayerChangeShape{player: (player_id,_),world: (_,world),shape,pos,..} if player_id == self.player_id => {
				self.move_time_count = 0.0;
				self.rotate_time_count = 0.0;

				self.recalculate_optimal_target(&*world,shape,pos);
			},
			_ => ()
		}
	}
}

#[allow(unused)]
fn world_optimality<W>(world: &W) -> f32
	where W: Grid,
	      <W as Grid>::Cell: Cell + Copy
{
	let mut o = 0.0;
	let world_height = world.height();

	for row in grid::rows_iter::Iter::new(world){
		let y = row.y;
		let height = world_height - y;
		let penalty = height as f32 * 20.0;

		for (x,cell) in grid::row::Iter::new(row){
			if cell.is_occupied(){
				o+= height as f32 * 3.0;
			}else if let Some(cell) = world.position(grid::Pos{x: x as grid::PosAxis,y: y as grid::PosAxis - 1}){
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
fn world_optimality2<W>(world: &W) -> f32
	where W: Grid,
	      <W as Grid>::Cell: Cell + Copy
{
	let world_height = world.height();
	let rows_completed = grid::rows_iter::Iter::new(world).filter_map(|row| if grid::row::Iter::new(row).all(|(_,cell)| cell.is_occupied()){Some(())}else{None}).count();
	let mut columns_height_sum = 0;
	let mut cells_vertically_blocked_penalty = 0.0;
	let mut height_bumpiness = 0;

	let mut previous_height = None::<grid::SizeAxis>;

	//Iterating columns
	for column in grid::columns_iter::Iter::new(world){
		let mut column = grid::column::Iter::new(column);

		//Find height (First occurence of a occupied cell)
		let height = if let Some((y,_)) = column.find(|&(_,cell)| cell.is_occupied()){
			let height = world_height - y;

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
