use super::super::grid::SizeAxis;
use super::{row,Grid};

pub struct Iter<'g,Grid: 'g>{
    grid: &'g Grid,
    row: SizeAxis,
}

impl<'g,G: Grid> Iter<'g,G>{
    pub fn new(grid: &'g G) -> Self{Iter{grid: grid,row: 0}}
}

impl<'g,G: Grid> Iterator for Iter<'g,G>
    where G: Grid,
          G::Cell: Copy
{
    type Item = row::Grid<'g,G>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.row < self.grid.height(){
            let y = self.row;
            self.row+=1;
            Some(row::Grid{grid: self.grid,row: y})
        }else{
            None
        }
    }
}
