use super::super::grid::Size;
use super::Grid;

pub struct PositionedCellIter<'g,Grid: 'g>{
    map: &'g Grid,
    pos: Size,
}

impl<'g,G: Grid> PositionedCellIter<'g,G>{
    pub fn new(map: &'g G) -> Self{PositionedCellIter{map: map,pos: Size{x: 0,y: 0}}}
}

impl<'g,G: Grid> Iterator for PositionedCellIter<'g,G>
    where G: Grid,
          G::Cell: Copy
{
    type Item = (Size,G::Cell);

    fn next(&mut self) -> Option<<Self as Iterator>::Item>{
        if self.pos.x == self.map.width(){
            self.pos.x = 0;
            self.pos.y+= 1;
        }

        if self.pos.y == self.map.height(){
            return None
        }

        let x = self.pos.x;
        self.pos.x+=1;

        return Some((Size{x: x,y: self.pos.y},unsafe{self.map.pos(x as usize,self.pos.y as usize)}));
    }
}
