pub mod ai;



use piston::event;

use data::player::Player;
use gamestate;

pub trait Controller<Map>{
	fn update(&mut self,args: &event::UpdateArgs,player: &mut Player,map: &mut Map);
	fn event(&mut self,event: gamestate::Event,player: &mut Player,map: &mut Map);
}
