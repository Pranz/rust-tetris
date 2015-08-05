use collections::vec_map::VecMap;
use core::cmp;
use piston::event;
use rand::{self,Rand};
use std::collections::hash_map::HashMap;

use super::data::cell::Cell;
use super::data::grid::{self,Grid};
use super::data::map;
use super::data::map::Map as MapTrait;
use super::data::player::{self,Player};
use super::data::shapes::tetrimino::{Shape,RotatedShape};
use super::game::event::Event;
use super::tmp_ptr::TmpPtr;

///Type of the map id
pub type MapId    = u8;
///Type of the player id
pub type PlayerId = u8;

#[derive(Copy,Clone,Debug,Eq,PartialEq,Hash)]
pub enum RngMapping{
    Map(MapId),
    Player(PlayerId)
}

///The ingame game state
pub struct GameState<Map,Rng>
    where Map: MapTrait
{
    ///Map pairs of maps and map ids
    pub maps          : VecMap<Map>,

    ///Map pairs of players and player ids
    pub players       : VecMap<Player>,

    pub rngs          : (Rng,HashMap<RngMapping,Rng>),

    ///Function that maps a shape's cell to the map's cell
    pub imprint_cell  : fn(&RotatedShape) -> <Map as Grid>::Cell,

    ///Function that returns the origin position of a player based on shape and map
    pub respawn_pos   : fn(&RotatedShape,&Map) -> grid::Pos
}

impl<Map,Rng> GameState<Map,Rng>
    where Map: MapTrait
{
    ///A simple constructor
    pub fn new(
        rng: Rng,
        imprint_cell: fn(&RotatedShape) -> <Map as Grid>::Cell,
        respawn_pos : fn(&RotatedShape,&Map) -> grid::Pos
    ) -> Self{GameState{
        maps   : VecMap::new(),
        players: VecMap::new(),
        rngs   : (rng,HashMap::new()),
        imprint_cell: imprint_cell,
        respawn_pos: respawn_pos,
    }}

    ///Updates the game state
    pub fn update<EL>(&mut self, args: &event::UpdateArgs,event_listener: &mut EL)
        where Map: MapTrait,
              <Map as Grid>::Cell: Cell,
              Rng: rand::Rng,
              EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
    {
        //After action
        enum Action{
            None,
            ResetMap(MapId)
        }let mut action = Action::None;

        //Players
        'player_loop: for (player_id,player) in self.players.iter_mut(){
            let player_id = player_id  as PlayerId;
            let map_id    = player.map as MapId;

            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                //Add the time since the last update to the time counts
                player.gravityfall_time_count += args.dt;
                player.slowfall_time_count    += args.dt;
                player.move_time_count        += args.dt;

                //If the time count is bigger than the shape move frequency, then repeat until it is smaller
                while player.gravityfall_time_count >= player.settings.gravityfall_frequency{
                    //Subtract one step of frequency
                    player.gravityfall_time_count -= player.settings.gravityfall_frequency;

                    //If able to move (no collision below)
                    if move_player(player,map,grid::Pos{x: 0,y: 1}){
                        event_listener(Event::PlayerMoveGravity{
                            player: (player_id,TmpPtr::new(player as &_)),
                            map: (map_id,TmpPtr::new(map as &_))
                        });
                    }else{
                        //Imprint the current shape onto the map
                        map.imprint_shape(&player.shape,player.pos,&self.imprint_cell);

                        //Handles the filled rows
                        let min_y = cmp::max(0,player.pos.y) as grid::SizeAxis;
                        let max_y = cmp::min(min_y + player.shape.height(),map.height());
                        let full_rows = if min_y!=max_y{
                            map.handle_full_rows(min_y .. max_y)
                        }else{
                            0
                        };

                        event_listener(Event::MapImprintShape{
                            map: (map_id,TmpPtr::new(map as &_)),
                            shape: (player.shape,player.pos),
                            full_rows: full_rows,
                            cause: Some((player_id,TmpPtr::new(player as &_))),
                        });

                        //Respawn player and check for collision at spawn position
                        let shape = <Shape as Rand>::rand(rng(&mut self.rngs,RngMapping::Player(player_id)));
                        if !respawn_player((player_id,player),(map_id,map),shape,self.respawn_pos,event_listener){
                            action = Action::ResetMap(map_id);
                            break 'player_loop;
                        }
                    }
                }
            }
        }

        match action{
            Action::None => (),
            Action::ResetMap(map_id) => self.reset_map(map_id,event_listener),
        };
    }

    ///Adds a player to the specified map and with the specified player settings
    ///Returns the new player id
    pub fn add_player<EL>(&mut self,map_id: MapId,settings: player::Settings,event_listener: &mut EL) -> Option<PlayerId>
        where Map: MapTrait,
              Rng: rand::Rng,
              EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
    {
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            let new_id = self.players.len();
            let shape = RotatedShape::new(<Shape as rand::Rand>::rand(rng(&mut self.rngs,RngMapping::Player(new_id as PlayerId))));

            self.players.insert(new_id,Player{
                pos                   : (self.respawn_pos)(&shape,map),
                shadow_pos            : None,
                shape                 : shape,
                map                   : map_id,
                points                : 0,
                gravityfall_time_count: 0.0,
                slowfall_time_count   : 0.0,
                move_time_count       : 0.0,
                settings              : settings
            });
            let player = self.players.get_mut(&new_id).unwrap();

            event_listener(Event::PlayerAdd{
                player: (new_id as PlayerId,TmpPtr::new(player as &_)),
                map: (map_id,TmpPtr::new(map as &_)),
            });

            Some(new_id as PlayerId)
        }else{
            None
        }
    }

    ///Resets the specified map, respawning all players and resetting time counts
    pub fn reset_map<EL>(&mut self,map_id: MapId,event_listener: &mut EL)
        where Map: MapTrait,
              <Map as Grid>::Cell: Cell,
              Rng: rand::Rng,
              EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
    {
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            //Clear map
            map.clear();

            for (player_id,player) in self.players.iter_mut().filter(|&(_,ref player)| player.map == map_id){
                //Reset all players in the map
                respawn_player((player_id as PlayerId,player),(map_id,map),<Shape as Rand>::rand(rng(&mut self.rngs,RngMapping::Player(player_id as PlayerId))),self.respawn_pos,event_listener);
                player.gravityfall_time_count = 0.0;
                player.slowfall_time_count    = 0.0;
                player.move_time_count        = 0.0;
            }
        };
    }
}

pub fn rng<Rng>(rngs: &mut (Rng,HashMap<RngMapping,Rng>),mapping: RngMapping) -> &mut Rng{
    match rngs.1.get_mut(&mapping){
        Some(rng) => rng,
        None => &mut rngs.0
    }
}

///Moves player if there are no collisions at the new position.
///Returns whether the movement was successful or not due to collisions.
pub fn move_player<Map>(player: &mut Player,map: &Map,delta: grid::Pos) -> bool
    where Map: MapTrait
{
    //Collision check
    match map.shape_intersects(&player.shape,grid::Pos{x: player.pos.x + delta.x,y: player.pos.y + delta.y}){
        //Collided => cannot move
        map::CellIntersection::Imprint(_) |
        map::CellIntersection::OutOfBounds(_) => false,

        //No collision, able to move and does so
        map::CellIntersection::None => {
            //Change position
            player.pos.x += delta.x;
            player.pos.y += delta.y;

            //Recalcuate fastfall shadow position when moving horizontally
            if player.settings.fastfall_shadow && delta.x!=0{
                player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,map,player.pos));
            }

            true
        }
    }
}

///Checks if the player with the transformed shape is intersecting with the map or the map boundaries.
///If that is true, try to resolve the collision by moving in the x axis.
///If the collision cannot resolve, undo the rotation and return false, otherwise return true.
pub fn resolve_transformed_player<Map>(player: &mut Player,shape: RotatedShape,map: &Map) -> bool
    where Map: MapTrait
{
    'try_rotate: loop{
        match map.shape_intersects(&shape,player.pos){
            map::CellIntersection::Imprint(pos) |
            map::CellIntersection::OutOfBounds(pos) => {
                let center_x = player.pos.x + player.shape.center_x() as grid::PosAxis;
                let sign = if pos.x < center_x {1} else {-1};
                for i in 1..shape.width(){
                    if let map::CellIntersection::None = map.shape_intersects(&shape,grid::Pos{x: player.pos.x + (i as grid::PosAxis * sign),y: player.pos.y}){
                        player.pos.x += i as grid::PosAxis * sign;
                        break 'try_rotate;
                    }
                }
            },
            _ => break 'try_rotate
        }

        return false;
    }

    {//Successfully rotated
        player.shape = shape;

        //Recalcuate fastfall shadow position when moving horizontally
        if player.settings.fastfall_shadow{
            player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,map,player.pos));
        }
        return true;
    }
}

///Respawns player to its origin position
///Returns whether the respawning was successful or not due to collisions.
pub fn respawn_player<Map,EL>((player_id,player): (PlayerId,&mut Player),(map_id,map): (MapId,&Map),new_shape: Shape,respawn_pos: fn(&RotatedShape,&Map) -> grid::Pos,event_listener: &mut EL) -> bool
    where Map: MapTrait,
          EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
{
    //Select a new shape at random, setting its position to the starting position
    let pos = respawn_pos(&player.shape,map);

    event_listener(Event::PlayerChangeShape{
        player: (player_id,TmpPtr::new(player as &_)),
        map: (map_id,TmpPtr::new(map)),
        shape: new_shape,
        pos: pos,
    });

    player.shape = RotatedShape::new(new_shape);
    player.pos = pos;

    if player.settings.fastfall_shadow{
        player.shadow_pos = Some(fastfallen_shape_pos(&player.shape,map,player.pos));
    }

    //If the new shape at the starting position also collides with another shape
    match map.shape_intersects(&player.shape,player.pos){
        map::CellIntersection::Imprint(_) => false,
        _ => true
    }
}

///Returns the position of the shape if it were to fast fall downwards in the map at the given position
pub fn fastfallen_shape_pos<Map>(shape: &RotatedShape,map: &Map,shape_pos: grid::Pos) -> grid::Pos
    where Map: MapTrait
{
    for y in shape_pos.y .. map.height() as grid::PosAxis{
        match map.shape_intersects(&shape,grid::Pos{x: shape_pos.x,y: y+1}){
            map::CellIntersection::Imprint(_)     |
            map::CellIntersection::OutOfBounds(_) => return grid::Pos{x: shape_pos.x,y: y},
            _ => ()
        };
    }

    unreachable!()
}
