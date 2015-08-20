use data::grid;
use data::input::Input;
use data::map::Map;
use data::player::Player;
use gamestate;

///Performs an action based on the given input on a player in a map
pub fn perform<M>(input: Input,player: &mut Player,map: &M)
	where M: Map
{
	match input{
		Input::MoveLeft => {
			gamestate::move_player(player,map,grid::Pos{x: -1, y: 0});
		},
		Input::MoveRight => {
			gamestate::move_player(player,map,grid::Pos{x: 1, y: 0});
		},
		Input::SlowFall => {
			player.gravityfall_time_count = if gamestate::move_player(player,map,grid::Pos{x: 0,y: 1}){
				//Reset timer
				player.settings.gravityfall_frequency
			} else {
				//Set timer and make the player move in the next update step
				0.0
			};
		},
		Input::FastFall => {
			player.pos = gamestate::fastfallen_shape_pos(&player.shape, map, player.pos);
			player.gravityfall_time_count = 0.0;
		},
		Input::RotateAntiClockwise => {
			let shape = player.shape.rotated_anticlockwise();
			gamestate::resolve_transformed_player(player, shape, map);
		},
		Input::RotateClockwise => {
			let shape = player.shape.rotated_clockwise();
			gamestate::resolve_transformed_player(player, shape, map);
		},
		_ => (),
	}
}
