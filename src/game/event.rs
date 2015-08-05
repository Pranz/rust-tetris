use super::super::data::grid;
use super::super::data::shapes::tetrimino::{Shape,RotatedShape};

///Events which can occur ingame.
///These should get signaled by the game state and listened to by a event listener.
#[derive(Copy,Clone,Debug)]
pub enum Event<Player,Map>{
    //MapStart(Map),
    //MapUpdate(Map),
    //MapEnd(Map),
    PlayerAdd{
        player: Player,
        map: Map
    },
    //PlayerRemove(PlayerId,Map),
    //PlayerMapChange(PlayerId,Map,Map),
    //PlayerRotate(PlayerId),
    //PlayerRotateCollide(PlayerId,Map),
    //PlayerMove(PlayerId,Map,grid::PosAxis,grid::PosAxis),
    //PlayerMoveCollide(PlayerId,Map,grid::PosAxis,grid::PosAxis),
    PlayerMoveGravity{
        player: Player,
        map: Map,
    },
    MapImprintShape{
        map: Map,
        shape: (RotatedShape,grid::Pos),
        full_rows: grid::SizeAxis,
        cause: Option<Player>,
    },
    PlayerChangeShape{
        player: Player,
        map: Map,
        shape: Shape,
        pos: grid::Pos
    },
}
