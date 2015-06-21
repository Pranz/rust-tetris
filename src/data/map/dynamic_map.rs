use core::ops::Range;
use core::ptr;

use super::super::grid::Grid;
use super::super::shapes::tetrimino::ShapeVariant;
use super::Map as MapTrait;
use super::cell::Cell;

///Rectangular dynamic sized game map
pub struct Map<Cell>{
    slice : Box<[Cell]>,
    width : super::SizeAxis,
}

impl<Cell: Copy> Grid for Map<Cell>{
    type Cell = Cell;

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{self.width}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{(self.slice.len()/(self.width as usize)) as super::SizeAxis}

    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> <Self as Grid>::Cell{
        self.slice[x + y*(self.width as usize)]
    }
}

impl<Cell: super::cell::Cell + Copy> MapTrait for Map<Cell>{
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: <Self as Grid>::Cell){
        self.slice[x + y*(self.width as usize)] = state;
    }

    fn clear(&mut self){
        for cell in self.slice.iter_mut(){
            *cell = <Self as Grid>::Cell::empty();
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
                    self.copy_row(y,y + full_row_count);

                    //Continue to check if the row fully consist of occupied cells
                    if (0..self.width()).all(|x| unsafe{self.pos(x as usize,y as usize)}.is_occupied()){
                        full_row_count += 1;
                    }
                }

                //TODO: Use move_rows instead?

                //Simulate row gravity (Part 2)
                //This applies to the rest of the rows
                for y in (full_row_count .. y_check_start).rev(){
                    self.copy_row(y,y + full_row_count);
                }

                //Simulate row gravity (Part 3)
                //Clear the rows that has been affected by gravity
                self.clear_rows(0 .. full_row_count*2);

                return full_row_count;
            }
        }

        return full_row_count;
    }

    fn clear_row(&mut self,y: super::SizeAxis){
        debug_assert!(y < self.height());

        for i in self.width * y .. self.width * (y+1){
            self.slice[i as usize] = <Self as Grid>::Cell::empty();
        }
    }

    fn copy_row(&mut self,y_from: super::SizeAxis,y_to: super::SizeAxis){
        debug_assert!(y_from != y_to);
        debug_assert!(y_from < self.height());
        debug_assert!(y_to < self.height());

        unsafe{ptr::copy_nonoverlapping(
            &    self.slice[(self.width as usize) * (y_from as usize)],
            &mut self.slice[(self.width as usize) * (y_to as usize)],
            self.width as usize
        )};
    }

    fn move_row(&mut self,y_from: super::SizeAxis,y_to: super::SizeAxis){
        self.copy_row(y_from,y_to);
        self.clear_row(y_from);
    }

    fn shape_intersects(&self, shape: &ShapeVariant, x: super::PosAxis, y: super::PosAxis) -> super::CellIntersection{
        super::defaults::shape_intersects(self,shape,x,y)
    }
}

impl<Cell: super::cell::Cell + Copy> Map<Cell>{
    pub fn new(width: super::SizeAxis,height: super::SizeAxis) -> Self{
        use core::iter::{self,FromIterator};

        Map{
            slice : Vec::from_iter(iter::repeat(<Self as Grid>::Cell::empty()).take((width as usize)*(height as usize))).into_boxed_slice(),
            width : width,
        }
    }

    pub fn clear_rows(&mut self,y: Range<super::SizeAxis>){
        debug_assert!(y.start < y.end);
        debug_assert!(y.end <= self.height());

        for i in self.width * y.start .. self.width * y.end{
            self.slice[i as usize] = <Self as Grid>::Cell::empty();
        }
    }

    pub fn move_rows(&mut self,y: Range<super::SizeAxis>,steps: super::PosAxis){
        debug_assert!(y.start < y.end);
        debug_assert!(y.end <= self.height());
        debug_assert!(steps != 0);
        debug_assert!(y.start as super::PosAxis + steps > 0);
        debug_assert!(y.end   as super::PosAxis + steps < self.height() as super::PosAxis);

        let src  = (self.width as usize) * (y.start as usize);
        let dest = (self.width as usize) * ((y.start as super::PosAxis + steps) as usize);
        let size = self.width as usize * y.len();

        if steps.abs() < y.len() as super::PosAxis{
            unsafe{ptr::copy(&self.slice[src],&mut self.slice[dest],size)};

            if steps > 0{
                self.clear_rows(y.start .. y.start + steps as super::SizeAxis);
            }else{
                self.clear_rows(y.start + (-steps) as super::SizeAxis .. y.start);
            }
        }else{
            unsafe{ptr::copy_nonoverlapping(&self.slice[src],&mut self.slice[dest],size)};
            self.clear_rows(y);
        }
    }
}
