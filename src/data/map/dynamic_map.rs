use core::ops::Range;

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
            *cell = <Self as MapTrait>::Cell::empty();
        }
    }

    fn position(&self,x: super::PosAxis,y: super::PosAxis) -> Option<<Self as MapTrait>::Cell>{
        if self.is_position_out_of_bounds(x,y){
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    fn set_position(&mut self,x: super::PosAxis,y: super::PosAxis,state: <Self as MapTrait>::Cell) -> Result<(),()>{
        if self.is_position_out_of_bounds(x,y){
            Err(())
        }else{
            unsafe{self.set_pos(x as usize,y as usize,state)};
            Ok(())
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
        where F: Fn(&BlockVariant) -> <Self as MapTrait>::Cell
    {
        for i in 0 .. BLOCK_COUNT{
            for j in 0 .. BLOCK_COUNT{
                if block.collision_map()[j as usize][i as usize]{
                    self.set_position(x+(i as super::PosAxis),y+(j as super::PosAxis),cell_constructor(block)).ok();
                }
            }
        }
    }

    fn handle_full_rows(&mut self,y_check: Range<super::SizeAxis>) -> super::SizeAxis{
        debug_assert!(y_check.start < y_check.end);
        debug_assert!(y_check.end <= self.height());

        let y_check_start = y_check.start;
        let mut full_row_count: super::SizeAxis = 0;
        let mut y_check = y_check.rev();

        //For each row that should be checked
        while let Some(y) = y_check.next(){
            //Check if the row fully consist of occupied cells
            if (0..self.width()).all(|x| unsafe{self.pos(x as usize,y as usize)}.is_occupied()){
                //Goes into the this "full_row_count > 0" scope
                full_row_count = 1;

                //Continue the iteration of each row that should be checked
                while let Some(y) = y_check.next(){
                    //Simulate row gravity (Part 1)
                    //This applies to the rows that should be checked
                    unsafe{self.copy_row(y,y + full_row_count)};

                    //Continue to check if the row fully consist of occupied cells
                    if (0..self.width()).all(|x| unsafe{self.pos(x as usize,y as usize)}.is_occupied()){
                        full_row_count += 1;
                    }
                }

                //Simulate row gravity (Part 2)
                //This applies to the rest of the rows
                for y in (full_row_count .. y_check_start).rev(){
                    unsafe{self.copy_row(y,y + full_row_count)};
                }

                //Simulate row gravity (Part 3)
                //Clear the rows that has been affected by gravity
                for y in 0 .. full_row_count{
                    unsafe{self.clear_row(y)};
                }

                return full_row_count;
            }
        }

        return full_row_count;
    }

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{self.width}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{(self.slice.len()/(self.width as usize)) as super::SizeAxis}
}

impl Map{
    ///Returns the cell at the given position without checks
    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> <Self as MapTrait>::Cell{
        self.slice[x + y*(self.width as usize)]
    }

    ///Sets the cell at the given position without checks
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: <Self as MapTrait>::Cell){
        self.slice[x + y*(self.width as usize)] = state;
    }

    pub fn cells_positioned<'s>(&'s self) -> CellIter<'s,Self>{CellIter{map: self,x: 0,y: 0}}

    pub fn new(width: super::SizeAxis,height: super::SizeAxis) -> Self{
        use core::iter::{self,FromIterator};

        Map{
            slice : Vec::from_iter(iter::repeat(<Self as MapTrait>::Cell::empty()).take((width as usize)*(height as usize))).into_boxed_slice(),
            width : width,
        }
    }

    pub unsafe fn clear_row(&mut self,y: super::SizeAxis){
        debug_assert!(y < self.height());

        for i in self.width * y .. self.width * (y+1){
            self.slice[i as usize] = <Self as MapTrait>::Cell::empty();
        }
    }

    pub unsafe fn clear_rows(&mut self,y: Range<super::SizeAxis>){
        debug_assert!(y.start < y.end);
        debug_assert!(y.end <= self.height());

        for i in self.width * y.start .. self.width * y.end{
            self.slice[i as usize] = <Self as MapTrait>::Cell::empty();
        }
    }

    pub unsafe fn copy_row(&mut self,y_from: super::SizeAxis,y_to: super::SizeAxis){
        use core::{mem,ptr};

        debug_assert!(y_from != y_to);
        debug_assert!(y_from < self.height());
        debug_assert!(y_to < self.height());

        //TODO: Guarantee drop for overwritten cells
        ptr::copy_nonoverlapping(
            &    self.slice[(self.width as usize) * (y_from as usize)],
            &mut self.slice[(self.width as usize) * (y_to as usize)],
            self.width as usize * mem::size_of::<<Self as MapTrait>::Cell>()
        );
    }

    pub unsafe fn move_rows(&mut self,y: Range<super::SizeAxis>,steps: super::PosAxis){
        use core::{mem,ptr};

        debug_assert!(y.start < y.end);
        debug_assert!(y.end <= self.height());
        debug_assert!(steps != 0);
        debug_assert!(y.start as super::PosAxis + steps > 0);
        debug_assert!(y.end   as super::PosAxis + steps < self.height() as super::PosAxis);

        //TODO: Guarantee drop for overwritten cells
        let src  = (self.width as usize) * (y.start as usize);
        let dest = (self.width as usize) * ((y.start as super::PosAxis + steps) as usize);
        let size = self.width as usize * y.len() * mem::size_of::<<Self as MapTrait>::Cell>();

        if steps.abs() < y.len() as super::PosAxis{
            ptr::copy(&self.slice[src],&mut self.slice[dest],size);

            if steps > 0{
                self.clear_rows(y.start .. y.start + steps as super::SizeAxis);
            }else{
                self.clear_rows(y.start + (-steps) as super::SizeAxis .. y.start);
            }
        }else{
            ptr::copy_nonoverlapping(&self.slice[src],&mut self.slice[dest],size);
            self.clear_rows(y);
        }
    }
}

pub struct CellIter<'m,Map: 'm>{
    map: &'m Map,
    x: super::SizeAxis,
    y: super::SizeAxis,
}
impl<'m> Iterator for CellIter<'m,Map>{
    type Item = (super::SizeAxis,super::SizeAxis,<Map as MapTrait>::Cell);

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
