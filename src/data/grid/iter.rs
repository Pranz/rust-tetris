use super::super::map::SizeAxis;
use super::Grid;

pub struct PositionedCellIter<'g,Grid: 'g>{
    map: &'g Grid,
    x: SizeAxis,
    y: SizeAxis,
}

impl<'g,G: Grid> PositionedCellIter<'g,G>{
    pub fn new(map: &'g G) -> Self{PositionedCellIter{map: map,x: 0,y: 0}}
}

impl<'g,G: Grid> Iterator for PositionedCellIter<'g,G>
    where G: Grid,
          G::Cell: Copy
{
    type Item = (SizeAxis,SizeAxis,G::Cell);

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
