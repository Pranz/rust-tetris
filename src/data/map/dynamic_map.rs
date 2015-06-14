use super::super::shapes::tetrimino::{BlockVariant,BLOCK_COUNT};
use super::Map as MapTrait;
use super::cell::Cell;

///Rectangular game map
pub struct Map {
    slice : Box<[u8]>,
    width : u8,
}

impl MapTrait for Map{
    type Cell = u8;

    fn clear(&mut self){
        for cell in self.slice.iter_mut(){
            *cell = <u8 as Cell>::empty();
        }
    }

    fn position(&self,x: super::PosAxis,y: super::PosAxis) -> Option<u8>{
        if self.is_position_out_of_range(x,y){
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    fn set_position(&mut self,x: super::PosAxis,y: super::PosAxis,state: u8) -> bool{
        if self.is_position_out_of_range(x,y){
            false
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            true
        }
    }

    fn block_intersects(&self, block: &BlockVariant, x: super::PosAxis, y: super::PosAxis) -> Option<(super::PosAxis, super::PosAxis)> {
        for i in 0..BLOCK_COUNT{
            for j in 0..BLOCK_COUNT{
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
        where F: Fn(&BlockVariant) -> u8
    {
        for i in 0 .. BLOCK_COUNT{
            for j in 0 .. BLOCK_COUNT{
                if block.collision_map()[j as usize][i as usize]{
                    self.set_position(x+(i as super::PosAxis),y+(j as super::PosAxis),cell_constructor(block));
                }
            }
        }
    }

    fn handle_full_rows(&mut self, lowest_y: super::SizeAxis){
        // TODO: In case we need to move lines anywhere else, split this function into two.
        let lowest_y = if lowest_y >= self.height() {self.height() - 1}else{lowest_y};
        let mut terminated_rows: super::SizeAxis = 0;
        for i in 0..BLOCK_COUNT{
            let lowest_y = lowest_y - i as super::SizeAxis + terminated_rows;
            if (0..self.width()).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}.is_occupied()){
                terminated_rows += 1;
                for j in 0..lowest_y{
                    unsafe{self.copy_row(lowest_y - j - 1,lowest_y - j)};
                }
                unsafe{self.clear_row(0)};
            }
        }
    }

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{self.width}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{(self.slice.len()/(self.width as usize)) as u8}
}

impl Map{
    ///Returns the cell at the given position without checks
    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> u8{
        self.slice[x + y*(self.width as usize)]
    }

    ///Sets the cell at the given position without checks
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: u8){
        self.slice[x + y*(self.width as usize)] = state;
    }

    pub fn cells<'s>(&'s self) -> CellIter<'s,Self>{CellIter{map: self,x: 0,y: 0}}
    pub fn cells_mut<'s>(&'s self) -> CellIter<'s,Self>{CellIter{map: self,x: 0,y: 0}}

    pub fn new(width: super::SizeAxis,height: super::SizeAxis) -> Self{
        use core::iter::{self,FromIterator};

        Map{
            slice : Vec::from_iter(iter::repeat(<u8 as Cell>::empty()).take((width as usize)*(height as usize))).into_boxed_slice(),
            width : width,
        }
    }

    pub unsafe fn clear_row(&mut self,y: super::SizeAxis){
        debug_assert!(y < self.height());

        for i in self.width * y .. (self.width+1) * y{
            self.slice[i as usize] = <u8 as Cell>::empty();
        }
    }

    pub unsafe fn copy_row(&mut self,y_from: super::SizeAxis,y_to: super::SizeAxis){
        use core::{mem,ptr};

        debug_assert!(y_from != y_to);
        debug_assert!(y_from < self.height());
        debug_assert!(y_to < self.height());

        ptr::copy_nonoverlapping(
            &self.slice[(self.width as usize) * (y_from as usize)],
            &mut self.slice[(self.width as usize) * (y_to as usize)],
            self.width as usize * mem::size_of::<<Self as MapTrait>::Cell>()
        );
    }
}

pub struct CellIter<'m,Map: 'm>{
    map: &'m Map,
    x: super::SizeAxis,
    y: super::SizeAxis,
}
impl<'m> Iterator for CellIter<'m,Map>{
    type Item = (super::SizeAxis,super::SizeAxis,u8);

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.x == self.map.width(){
            self.x = 0;
            self.y+= 1;
        }

        if self.y == self.map.height(){
            return None
        }

        let x = self.x;
        self.x+=1;

        return Some((x,self.y,unsafe{self.map.pos(x as usize,self.y as usize)}));
    }
}
