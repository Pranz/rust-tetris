pub mod ai;



use piston::input::UpdateArgs;
use vec_map::VecMap;

use ::data::Player;

///Controlls a player and its world with inputs
pub trait Controller<World,Event>{
	///Called for each update step
	fn update(&mut self,args: &UpdateArgs,players: &VecMap<Player>,worlds: &VecMap<World>);

	///Event listener. Called for each defined ingame event occcurring
	fn event<'l>(&mut self,event: &Event);
}
