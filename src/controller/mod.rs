pub mod ai;



use collections::vec_map::VecMap;
use piston::event;

use data::player::Player;

///Controlls a player and its map with inputs
pub trait Controller<Map,Event>{
	///Called for each update step
	fn update(&mut self,args: &event::UpdateArgs,players: &VecMap<Player>,maps: &VecMap<Map>);

	///Event listener. Called for each defined ingame event occcurring
	fn event(&mut self,event: Event);
}
