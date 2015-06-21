use core::ops::Range;

use super::super::grid::Grid;
use super::super::shapes::tetrimino::ShapeVariant;
use super::Map as MapTrait;

///Constant width of the map
const WIDTH : super::SizeAxis = 10;

///Constant height of the map
const HEIGHT: super::SizeAxis = 20;

///Rectangular static sized game map
pub struct Map<Cell>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: Copy> Grid for Map<Cell>{
    type Cell = Cell;

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{WIDTH}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{HEIGHT}

    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> Cell{
        self.0[y][x]
    }
}

impl<Cell: super::cell::Cell + Copy> MapTrait for Map<Cell>{
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
        self.0[y][x] = state;
    }

    fn handle_full_rows(&mut self,y_check: Range<super::SizeAxis>) -> super::SizeAxis{
        debug_assert!(y_check.start < y_check.end);
        debug_assert!(y_check.end <= HEIGHT);

        let mut terminated_rows: super::SizeAxis = 0;

        for y_lowest in y_check.rev(){
            let y_lowest = y_lowest + terminated_rows;
            if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,y_lowest as usize)}.is_occupied()){
                terminated_rows += 1;
                for y in (0..y_lowest).rev(){
                    self.copy_row(y,y+1);
                }
                self.clear_row(0);
            }
        }

        return terminated_rows;
    }

    #[inline(always)]
    fn clear_row(&mut self,y: super::SizeAxis){
        debug_assert!(y < self.height());

        self.0[y as usize] = [Cell::empty(); WIDTH as usize];
    }

    #[inline(always)]
    fn copy_row(&mut self,y_from: super::SizeAxis,y_to: super::SizeAxis){
        debug_assert!(y_from != y_to);
        debug_assert!(y_from < self.height());
        debug_assert!(y_to < self.height());

        self.0[y_from as usize] = self.0[y_to as usize];
    }

    fn shape_intersects(&self, shape: &ShapeVariant, x: super::PosAxis, y: super::PosAxis) -> super::CellIntersection{
        super::defaults::shape_intersects(self,shape,x,y)
    }
}

impl<Cell: super::cell::Cell + Copy> Default for Map<Cell>{
    fn default() -> Self{
        Map([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
    }
}
