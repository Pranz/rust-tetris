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

pub mod key{
    use piston::input::Key;
    use std::collections::hash_map::HashMap;

    use data::input::Input;
    use gamestate::PlayerId;

    pub type KeyMap = HashMap<Key,Mapping>;

    #[derive(Copy,Clone,PartialEq)]
    pub struct Mapping{
        pub player          : PlayerId,
        pub input           : Input,
        pub repeat_delay    : f64,//Unit: seconds
        pub repeat_frequency: f64,//Unit: seconds/trigger
    }

    /*
    pub slowfall_time_count   : f64,//Unit: seconds
    pub move_time_count       : f64,//Unit: seconds

    pub slowfall_delay       : f64,//Unit: seconds
    pub slowfall_frequency   : f64,//Unit: seconds/block
    pub move_delay           : f64,//Unit: seconds
    pub move_frequency       : f64,//Unit: seconds/block
     */

}
