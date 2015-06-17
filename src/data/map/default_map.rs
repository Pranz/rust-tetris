use core::ops::Range;

use super::super::shapes::tetrimino::{BlockVariant,BLOCK_COUNT};
use super::Map as MapTrait;

///Constant width of the map
const WIDTH : super::SizeAxis = 10;

///Constant height of the map
const HEIGHT: super::SizeAxis = 20;

///Rectangular game map
pub struct Map<Cell>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: super::cell::Cell + Copy> MapTrait for Map<Cell>{
    type Cell = Cell;

    fn clear(&mut self){
        for y in 0..self.height(){
            for x in 0..self.width(){
                unsafe{self.set_pos(x as usize,y as usize,Cell::empty())};
            }
        }
    }

    fn position(&self,x: super::PosAxis,y: super::PosAxis) -> Option<Cell>{
        if self.is_position_out_of_bounds(x,y){
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    fn set_position(&mut self,x: super::PosAxis,y: super::PosAxis,state: Cell) -> Result<(),()>{
        if self.is_position_out_of_bounds(x,y){
            Err(())
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            Ok(())
        }
    }

    fn block_intersects(&self, block: &BlockVariant, x: super::PosAxis, y: super::PosAxis) -> Option<(super::PosAxis, super::PosAxis)> {
        for j in 0..BLOCK_COUNT{
            for i in 0..BLOCK_COUNT{
                if block.collision_map()[j as usize][i as usize] {
                    let (x,y) = (i as super::PosAxis + x,j as super::PosAxis + y);
                    match self.position(x,y){
                        None                           => return Some((x,y)),
                        Some(pos) if pos.is_occupied() => return Some((x,y)),
                        _ => ()
                    };
                }
            }
        }
        None
    }

    fn imprint_block<F>(&mut self,block: &BlockVariant, x: super::PosAxis, y: super::PosAxis,cell_constructor: F)
        where F: Fn(&BlockVariant) -> Cell
    {
        for j in 0 .. BLOCK_COUNT{
            for i in 0 .. BLOCK_COUNT{
                if block.collision_map()[j as usize][i as usize]{
                    self.set_position(x+(i as super::PosAxis),y+(j as super::PosAxis),cell_constructor(block)).ok();
                }
            }
        }
    }

    fn handle_full_rows(&mut self,y_check: Range<super::SizeAxis>) -> super::SizeAxis{
        // TODO: In case we need to move lines anywhere else, split this function into two.
        debug_assert!(y_check.start < y_check.end);
        debug_assert!(y_check.end <= HEIGHT);

        let mut terminated_rows: super::SizeAxis = 0;
        for y_lowest in y_check.rev(){
            let y_lowest = y_lowest + terminated_rows;
            if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,y_lowest as usize)}.is_occupied()){
                terminated_rows += 1;
                for y in (0..y_lowest).rev(){
                    self.0[y as usize + 1] = self.0[y as usize];
                }
                self.0[0] = [Cell::empty(); WIDTH as usize];
            }
        }

        return terminated_rows;
    }

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{WIDTH}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{HEIGHT}
}

impl<Cell: Copy> Map<Cell>{
    ///Returns the cell at the given position without checks
    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> Cell{
        self.0[y][x]
    }

    ///Sets the cell at the given position without checks
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
        self.0[y][x] = state;
    }

    pub fn cells_positioned<'s>(&'s self) -> CellIter<'s,Self>{CellIter{map: self,x: 0,y: 0}}
}


impl<Cell: super::cell::Cell + Copy> Default for Map<Cell>{
    fn default() -> Self{
        Map([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
    }
}

pub struct CellIter<'m,Map: 'm>{
    map: &'m Map,
    x: super::SizeAxis,
    y: super::SizeAxis,
}
impl<'m,Cell> Iterator for CellIter<'m,Map<Cell>>
    where Cell: Copy
{
    type Item = (super::SizeAxis,super::SizeAxis,Cell);

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.x == WIDTH{
            self.x = 0;
            self.y+= 1;
        }

        if self.y == HEIGHT{
            return None
        }

        let x = self.x;
        self.x+=1;

        return Some((x,self.y,unsafe{self.map.pos(x as usize,self.y as usize)}));
    }
}
