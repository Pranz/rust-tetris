use collections::vec_map::VecMap;
use piston::event;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::input::Input;
use data::player::Player;
use data::map::Map;
use gamestate;
use game::event::Event;
use tmp_ptr::TmpPtr;

#[derive(Clone)]
pub struct Controller{
	pub input_sender: sync::mpsc::Sender<(Input,gamestate::PlayerId)>,
	pub player_id: gamestate::PlayerId,
	bounce: bool,
	move_time: f64,
}

impl Controller{
	pub fn new(input_sender: sync::mpsc::Sender<(Input,gamestate::PlayerId)>,player_id: gamestate::PlayerId) -> Self{Controller{
		input_sender: input_sender,
		player_id: player_id,
		bounce: false,
		move_time: 0.0,
	}}
}

impl<M: Map> ControllerTrait<M,Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::MapId,TmpPtr<M>)>> for Controller{
	fn update(&mut self,args: &event::UpdateArgs,_: &VecMap<Player>,_: &VecMap<M>){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			let _ = self.input_sender.send((if self.bounce{Input::MoveLeft}else{Input::MoveRight},self.player_id));
			self.move_time -= 0.3;
		}
	}

	fn event(&mut self,_: Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::MapId,TmpPtr<M>)>){/*
		use game::event::Event::*;

		match event{
			TODO: PlayerMoveCollide{..} => {self.bounce = !self.bounce;}
		}
	*/}
}
