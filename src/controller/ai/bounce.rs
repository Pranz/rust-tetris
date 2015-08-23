use vec_map::VecMap;
use piston::input::UpdateArgs;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::{Input,Player,Request,World};
use game::Event;
use gamestate;
use tmp_ptr::TmpPtr;

#[derive(Clone)]
pub struct Controller{
	pub request_sender: sync::mpsc::Sender<Request>,
	pub player_id: gamestate::PlayerId,
	bounce: bool,
	move_time: f64,
}

impl Controller{
	pub fn new(request_sender: sync::mpsc::Sender<Request>,player_id: gamestate::PlayerId) -> Self{Controller{
		request_sender: request_sender,
		player_id: player_id,
		bounce: false,
		move_time: 0.0,
	}}
}

impl<W: World> ControllerTrait<W,Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::WorldId,TmpPtr<W>)>> for Controller{
	fn update(&mut self,args: &UpdateArgs,_: &VecMap<Player>,_: &VecMap<W>){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			let _ = self.request_sender.send(Request::Input{
				input: if self.bounce{Input::MoveLeft}else{Input::MoveRight},
				player: self.player_id
			});
			self.move_time -= 0.3;
		}
	}

	fn event(&mut self,_: Event<(gamestate::PlayerId,TmpPtr<Player>),(gamestate::WorldId,TmpPtr<W>)>){/*
		use game::Event::*;

		match event{
			TODO: PlayerMoveCollide{..} => {self.bounce = !self.bounce;}
		}
	*/}
}
