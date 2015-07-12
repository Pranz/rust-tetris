use super::super::grid::SizeAxis;
use super::{column,Grid};

pub struct Iter<'g,Grid: 'g>{
    grid: &'g Grid,
    column: SizeAxis,
}

impl<'g,G: Grid> Iter<'g,G>{
    pub fn new(grid: &'g G) -> Self{Iter{grid: grid,column: 0}}
}

impl<'g,G: Grid> Iterator for Iter<'g,G>
    where G: Grid,
          G::Cell: Copy
{
    type Item = column::Grid<'g,G>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.column < self.grid.width(){
            let x = self.column;
            self.column+=1;
            Some(column::Grid{grid: self.grid,column: x})
        }else{
            None
        }
    }
}
