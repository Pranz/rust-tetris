use core::fmt;

use super::{Grid,RectangularBound};
use ::data::Cell;

pub struct Printer<'g,Grid: 'g>(pub &'g Grid);

impl<'g,G: Grid> fmt::Debug for Printer<'g,G>
	where G: Grid + RectangularBound,
	      G::Cell: Copy + fmt::Debug{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		for row in super::rows_iter::Iter::new(self.0){
			for (_,cell) in super::row::Iter::new(row){
				try!(write!(f,"{:?} ",cell));
			}
			try!(write!(f,"\n"));
		}
		Ok(())
	}
}

pub struct OccupyPrinter<'g,Grid: 'g>(pub &'g Grid);

impl<'g,G: Grid> fmt::Debug for OccupyPrinter<'g,G>
	where G: Grid + RectangularBound,
	      G::Cell: Copy + fmt::Debug + Cell{
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
		for row in super::rows_iter::Iter::new(self.0){
			for (_,cell) in super::row::Iter::new(row){
				try!(write!(f,"{}",if cell.is_occupied(){'#'}else{'-'}));
			}
			try!(write!(f,"\n"));
		}
		Ok(())
	}
}
