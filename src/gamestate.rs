use collections::vec_map::VecMap;
use core::cmp;
use piston::event;
use rand::{self,Rand};

use super::data::cell;
use super::data::grid::{self,Grid};
use super::data::map;
use super::data::map::Map as MapTrait;
use super::data::player::{self,Player};
use super::data::shapes::tetrimino::{Shape,RotatedShape};
use super::game::event::Event;
use super::tmp_ptr::TmpPtr;

pub type MapId    = u8;
pub type PlayerId = u8;

pub struct GameState<Map,Rng>
{
    pub maps          : VecMap<Map>,
    pub players       : VecMap<Player>,
    pub rng           : Rng,//TODO: See http://doc.rust-lang.org/rand/src/rand/lib.rs.html#724-726 for getting a seed
    pub imprint_cell  : fn(&RotatedShape) -> cell::ShapeCell
}

impl<Map,Rng> GameState<Map,Rng>{
    pub fn new(rng: Rng,imprint_cell: fn(&RotatedShape) -> cell::ShapeCell) -> Self{GameState{
        maps   : VecMap::new(),
        players: VecMap::new(),
        rng    : rng,
        imprint_cell: imprint_cell,
    }}

    pub fn update<EL>(&mut self, args: &event::UpdateArgs,event_listener: &mut EL)
        where Map: MapTrait<Cell = cell::ShapeCell>,
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
                        let shape = <Shape as Rand>::rand(&mut self.rng);
                        if !respawn_player((player_id,player),(map_id,map),shape,event_listener){
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

    pub fn with_player<F: FnOnce(&mut Player) -> R,R>(&mut self,player_id: PlayerId,f: F) -> Option<R>{
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            return Some(f(player))
        }
        None
    }

    pub fn with_player_map<F: FnOnce(&mut Player,&mut Map)-> R,R>(&mut self,player_id: PlayerId,f: F) -> Option<R>{
        if let Some(player) = self.players.get_mut(&(player_id as usize)){
            if let Some(map) = self.maps.get_mut(&(player.map as usize)){
                return Some(f(player,map))
            }
        }
        None
    }

    pub fn add_player<EL>(&mut self,map_id: MapId,settings: player::Settings,event_listener: &mut EL) -> Option<PlayerId>
        where Map: MapTrait,
              Rng: rand::Rng,
              EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
    {
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            let new_id = self.players.len();
            let shape = RotatedShape::new(<Shape as rand::Rand>::rand(&mut self.rng));

            self.players.insert(new_id,Player{
                pos                   : respawn_position(shape,map),
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

    pub fn reset_map<EL>(&mut self,map_id: MapId,event_listener: &mut EL)
        where Map: MapTrait,
              <Map as Grid>::Cell: cell::Cell,
              Rng: rand::Rng,
              EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
    {
        if let Some(map) = self.maps.get_mut(&(map_id as usize)){
            //Clear map
            map.clear();

            for (player_id,player) in self.players.iter_mut().filter(|&(_,ref player)| player.map == map_id){
                //Reset all players in the map
                respawn_player((player_id as PlayerId,player),(map_id,map),<Shape as Rand>::rand(&mut self.rng),event_listener);
                player.gravityfall_time_count = 0.0;
            }
        };
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
                player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
            }

            true
        }
    }
}

///Tries to rotate. If this results in a collision, try to resolve this collision by
///moving in the x axis. If the collision cannot resolve, amend the rotation and return false,
///otherwise return true.
pub fn transform_resolve_player<Map>(player: &mut Player,shape: RotatedShape,map: &Map) -> bool
    where Map: MapTrait
{
    'try_rotate: loop{
        match map.shape_intersects(&shape,player.pos){
            map::CellIntersection::Imprint(pos) |
            map::CellIntersection::OutOfBounds(pos) => {
                let center_x = player.pos.x + player.shape.center_x() as grid::PosAxis;
                let sign = if pos.x < center_x {1} else {-1};
                for i in 1..player.shape.width(){//TODO: Should this check the player's shape? (The old hsape)
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
            player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
        }
        return true;
    }
}

///Returns the origin position based on the player and map
pub fn respawn_position<Map>(shape: RotatedShape,map: &Map) -> grid::Pos
    where Map: MapTrait
{
    grid::Pos{
        x: map.width() as grid::PosAxis/2 - shape.center_x() as grid::PosAxis,
        y: 0//TODO: Spawn above optionally: -(player.shape.height() as grid::PosAxis);
    }
}

///Respawns player to its origin position
///Returns whether the respawning was successful or not due to collisions.
pub fn respawn_player<Map,EL>((player_id,player): (PlayerId,&mut Player),(map_id,map): (MapId,&Map),new_shape: Shape,event_listener: &mut EL) -> bool
    where Map: MapTrait,
          EL: FnMut(Event<(PlayerId,TmpPtr<Player>),(MapId,TmpPtr<Map>)>)
{
    //Select a new shape at random, setting its position to the starting position
    let pos = respawn_position(player.shape,map);

    event_listener(Event::PlayerChangeShape{
        player: (player_id,TmpPtr::new(player as &_)),
        map: (map_id,TmpPtr::new(map)),
        shape: new_shape,
        pos: pos,
    });

    player.shape = RotatedShape::new(new_shape);
    player.pos = pos;

    if player.settings.fastfall_shadow{
        player.shadow_pos = Some(fast_fallen_shape(&player.shape,map,player.pos));
    }

    //If the new shape at the starting position also collides with another shape
    match map.shape_intersects(&player.shape,player.pos){
        map::CellIntersection::Imprint(_) => false,
        _ => true
    }
}

pub fn fast_fallen_shape<Map>(shape: &RotatedShape,map: &Map,shape_pos: grid::Pos) -> grid::Pos
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
