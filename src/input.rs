//! Input code

use data::{grid,Input,Player,World};
use gamestate;

///Performs an action based on the given input on a player in a world
pub fn perform<W>(input: Input,player: &mut Player,world: &W)
	where W: World
{
	match input{
		Input::MoveLeft => {
			gamestate::move_player(player,world,grid::Pos{x: -1, y: 0});
		},
		Input::MoveRight => {
			gamestate::move_player(player,world,grid::Pos{x: 1, y: 0});
		},
		Input::SlowFall => {
			player.gravityfall_time_count = if gamestate::move_player(player,world,grid::Pos{x: 0,y: 1}){
				//Reset timer
				player.settings.gravityfall_frequency
			} else {
				//Set timer and make the player move in the next update step
				0.0
			};
		},
		Input::FastFall => {
			player.pos = gamestate::fastfallen_shape_pos(&player.shape, world, player.pos);
			player.gravityfall_time_count = 0.0;
		},
		Input::RotateAntiClockwise => {
			let shape = player.shape.rotated_anticlockwise();
			gamestate::resolve_transformed_player(player, shape, world);
		},
		Input::RotateClockwise => {
			let shape = player.shape.rotated_clockwise();
			gamestate::resolve_transformed_player(player, shape, world);
		},
		_ => (),
	}
}
