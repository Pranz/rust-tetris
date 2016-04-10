pub mod ai;



use piston::input::UpdateArgs;

use ::game;

///Controlls a player and its world with inputs
pub trait Controller<World,Event>{
	///Called for each update step
	fn update(&mut self,args: &UpdateArgs,game_data: &game::Data<World>);

	///Event listener. Called for each defined ingame event occcurring
	fn event<'l>(&mut self,event: &Event);
}
