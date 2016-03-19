use vec_map::VecMap;
use piston::input::UpdateArgs;
use std::sync;

use super::super::Controller as ControllerTrait;
use data::{Input,Player,Request,World};
use game::Event;
use gamestate;

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

impl<W> ControllerTrait<W,Event<gamestate::PlayerId,gamestate::WorldId>> for Controller
	where W: World
{
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

	fn event<'l>(&mut self,event: &Event<gamestate::PlayerId,gamestate::WorldId>){/*
		use game::Event::*;

		match event{
			TODO: PlayerMoveCollide{..} => {self.bounce = !self.bounce;}
		}
	*/}
}
