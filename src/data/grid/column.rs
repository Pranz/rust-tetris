use super::super::cell::Cell;
use super::Grid as GridTrait;
use super::{PosAxis,SizeAxis,Pos};

pub struct Grid<'g,G: 'g>{
    pub grid: &'g G,
    pub column: SizeAxis
}

impl<'g,G> GridTrait for Grid<'g,G>
    where G: GridTrait + 'g,
          <G as GridTrait>::Cell: Copy
{
    type Cell = <G as GridTrait>::Cell;

    fn is_position_out_of_bounds(&self,pos: Pos) -> bool{
        if pos.x == self.column as PosAxis{
            self.grid.is_position_out_of_bounds(pos)
        }else{
            false
        }
    }

    fn width(&self) -> SizeAxis{1}
    fn height(&self) -> SizeAxis{self.grid.height()}

    unsafe fn pos(&self,x: usize,y: usize) -> Self::Cell{
        self.grid.pos(x,y)
    }
}

pub struct Iter<'g,G: 'g>{
    grid: Grid<'g,G>,
    row: SizeAxis
}

impl<'g,G> Iter<'g,G>
    where G: GridTrait + 'g,
{
    pub fn new(grid: Grid<'g,G>) -> Self{Iter{grid: grid,row: 0}}
}

impl<'g,G> Iterator for Iter<'g,G>
    where G: GridTrait + 'g,
          <G as GridTrait>::Cell: Copy
{
    type Item = (SizeAxis,<G as GridTrait>::Cell);

    fn next(&mut self) -> Option<Self::Item>{
        if let Some(cell) = self.grid.position(Pos{x: self.grid.column as PosAxis,y: self.row as PosAxis}){
            let row = self.row;
            self.row+= 1;
            Some((row,cell))
        }else{
            None
        }
    }
}
