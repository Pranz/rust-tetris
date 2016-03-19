use collections::borrow::Borrow;
use core::marker::PhantomData;
use serde::{Serialize,Serializer};
use serde::ser::impls::SeqIteratorVisitor;

use ::data::{Cell,Grid};
use ::data::grid::cells_iter::Iter as CellIter;
use ::data::grid::RectangularBound;

pub struct GridSerializer<B: Borrow<G>,G>(B,PhantomData<G>);

impl<B,G> GridSerializer<B,G>
	where B: Borrow<G>,
	      G: Grid + RectangularBound,
	      <G as Grid>::Cell: Cell + Copy
{
	#[inline]pub fn new(world: B) -> Self{GridSerializer(world,PhantomData)}

	pub fn visit<S>(self,serializer: &mut S) -> Result<(),S::Error>
		where S: Serializer
	{
		let grid = self.0.borrow();
		let size = grid.size();

		try!(size.x.serialize(serializer));
		try!(size.y.serialize(serializer));
		serializer.serialize_seq(SeqIteratorVisitor::new(CellIter::new(grid).map(|(_,cell)| cell.is_occupied()),Some(size.x as usize * size.y as usize)))
	}
}
