use super::super::shapes::tetrimino::{BlockVariant,BLOCK_COUNT};
use super::Map as MapTrait;

///Constant width of the map
const WIDTH : super::SizeAxis = 10;

///Constant height of the map
const HEIGHT: super::SizeAxis = 20;

///Rectangular game map
pub struct Map<Cell>([[Cell; WIDTH as usize]; HEIGHT as usize]);

impl<Cell: super::cell::Cell + Copy> MapTrait for Map<Cell>{
    type Cell = Cell;

    fn clear(&mut self){
        for i in 0..self.width(){
            for j in 0..self.height(){
                self.set_position(i as super::PosAxis,j as super::PosAxis,Cell::empty());
            }
        }
    }

    fn position(&self,x: super::PosAxis,y: super::PosAxis) -> Option<Cell>{
        if self.is_position_out_of_range(x,y){
            None
        }else{
            Some(unsafe{self.pos(x as usize,y as usize)})
        }
    }

    fn set_position(&mut self,x: super::PosAxis,y: super::PosAxis,state: Cell) -> bool{
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
        where F: Fn(&BlockVariant) -> Cell
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
        let lowest_y = if lowest_y >= HEIGHT{HEIGHT - 1}else{lowest_y};
        let mut terminated_rows: super::SizeAxis = 0;
        for i in 0..BLOCK_COUNT{
            let lowest_y = lowest_y - i as super::SizeAxis + terminated_rows;
            if (0..WIDTH).all(|x| unsafe{self.pos(x as usize,lowest_y as usize)}.is_occupied()){
                terminated_rows += 1;
                for j in 0..lowest_y{
                    self.0[(lowest_y - j) as usize] = self.0[(lowest_y - j - 1) as usize];
                }
                self.0[0] = [Cell::empty(); WIDTH as usize];
            }
        }
    }

    #[inline(always)]
    fn width(&self) -> super::SizeAxis{WIDTH}

    #[inline(always)]
    fn height(&self) -> super::SizeAxis{HEIGHT}
}

impl<Cell: Copy> Map<Cell>{
    ///Returns the cell at the given position without checks
    #[inline(always)]
    unsafe fn pos(&self,x: usize,y: usize) -> Cell{
        self.0[y][x]
    }

    ///Sets the cell at the given position without checks
    #[inline(always)]
    unsafe fn set_pos(&mut self,x: usize,y: usize,state: Cell){
        self.0[y][x] = state;
    }

    pub fn cells<'s>(&'s self) -> CellIter<'s,Self>{CellIter{map: self,x: 0,y: 0}}
}


impl<Cell: super::cell::Cell + Copy> Default for Map<Cell>{
    fn default() -> Self{
        Map([[Cell::empty(); WIDTH as usize]; HEIGHT as usize])
    }
}

pub struct CellIter<'m,Map: 'm>{
	map: &'m Map,
	x: super::SizeAxis,
	y: super::SizeAxis,
}
impl<'m,Cell> Iterator for CellIter<'m,Map<Cell>>
    where Cell: Copy
{
	type Item = (super::SizeAxis,super::SizeAxis,Cell);

	fn next(&mut self) -> Option<<Self as Iterator>::Item>{
		if self.x == WIDTH{
			self.x = 0;
			self.y+= 1;
		}

		if self.y == HEIGHT{
			return None
		}

		let x = self.x;
		self.x+=1;

		return Some((x,self.y,unsafe{self.map.pos(x as usize,self.y as usize)}));
	}
}
