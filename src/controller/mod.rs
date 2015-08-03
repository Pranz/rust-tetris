pub mod ai;



use collections::vec_map::VecMap;
use piston::event;

use data::player::Player;

///Controlls a player and its map
pub trait Controller<Map,Event>{
	fn update(&mut self,args: &event::UpdateArgs,players: &VecMap<Player>,maps: &VecMap<Map>);
	fn event(&mut self,event: Event);
}
