use piston::input::UpdateArgs;
use std::sync;

use super::super::Controller as ControllerTrait;
use ::data::{Input,Request,World};
use ::game::{self,Event};
use ::game::data::{WorldId,PlayerId};

#[derive(Clone)]
pub struct Controller{
	pub request_sender: sync::mpsc::Sender<Request<PlayerId,WorldId>>,
	pub player_id: game::data::PlayerId,
	bounce: bool,
	move_time: f64,
}

impl Controller{
	pub fn new(request_sender: sync::mpsc::Sender<Request<PlayerId,WorldId>>,player_id: game::data::PlayerId) -> Self{Controller{
		request_sender: request_sender,
		player_id: player_id,
		bounce: false,
		move_time: 0.0,
	}}
}

impl<W> ControllerTrait<W,Event<game::data::PlayerId,game::data::WorldId>> for Controller
	where W: World
{
	fn update(&mut self,args: &UpdateArgs,_: &game::Data<W>){
		self.move_time+= args.dt;

		if self.move_time > 0.3{
			let _ = self.request_sender.send(Request::PlayerInput{
				input: if self.bounce{Input::MoveLeft}else{Input::MoveRight},
				player: self.player_id
			});
			self.move_time -= 0.3;
		}
	}

	fn event<'l>(&mut self,_: &Event<game::data::PlayerId,game::data::WorldId>){/*
		use game::Event::*;

		match event{
			TODO: PlayerMoveCollide{..} => {self.bounce = !self.bounce;}
		}
	*/}
}
