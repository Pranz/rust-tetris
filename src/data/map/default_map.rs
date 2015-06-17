use core::ops::Range;

use super::Map as MapTrait;

///Constant width of the map
const WIDTH : super::SizeAxis = 10;

///Constant height of the map
const HEIGHT: super::SizeAxis = 20;

///Rectangular static sized game map
pub struct Map<Cell>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: super::cell::Cell + Copy> MapTrait for Map<Cell>{
    type Cell = Cell;

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

    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> Cell{
        self.0[y][x]
    }

    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
        self.0[y][x] = state;
    }
}

impl<Cell: Copy> Map<Cell>{
    pub fn cells_positioned<'s>(&'s self) -> super::PositionedCellIter<'s,Self>{super::PositionedCellIter{map: self,x: 0,y: 0}}
}


impl<Cell: super::cell::Cell + Copy> Default for Map<Cell>{
    fn default() -> Self{
        Map([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
    }
}
