use std::iter;

#[derive(Copy,Clone,PartialEq)]
pub struct RectangularViews{
	///Number of views
	pub count: u16,

	///Ratio (width/height) of the views container (e.g. 4:3 becomes 4.0/3.0)
	pub ratio: f32
}

impl RectangularViews{
	///Number of rows
	pub fn rows(self) -> u16{
		//columns = views/rows
		//viewRatio = columns/rows = views/rows²
		//0 = ratio - viewRatio (Find the view ratio nearest 1.0. Solve for variable `rows`)
		//0 = ratio - views/rows²
		//views/rows² = ratio
		//rows² = views/ratio
		//rows = sqrt(views/ratio)
		(self.count as f32 / self.ratio).sqrt().round() as u16
	}

	///Number of columns for a specific row
	pub fn row_columns(self,row: u16) -> u16{
		let rows = self.rows();
		let columns_per_row = (self.count as f32) / (rows as f32);
		//to_distribute = fract(columns_per_row) * rows;
		//row_interval_to_distribute_to = rows / to_distribute
		//                              = rows / (fract(columns_per_row) * rows)
		//                              = 1.0 / fract(columns_per_row)
		//row_to_distribute_to(n) = n * row_interval_to_distribute_to
		//                        = n * 1.0 / fract(columns_per_row)
		//                        = n  / fract(columns_per_row)
		//columns_per_row is a whole number => fract(columns_per_row)=0 => Evenly distributed
		//When evenly distributed, don't handle the extra distributing,
		//else; check whether the current row number is in the sequence of rows to distribute to.
		(columns_per_row.trunc() as u16) + (if columns_per_row.fract()!=0.0 && ((row as f32) % (1.0/columns_per_row.fract())).trunc() as u16 == 0{1}else{0})
	}

	///Iterator through all rows, with an amount of columns for each row
	pub fn layout(self) -> LayoutIter{
		let rows = self.rows();
		let columns_per_row = (self.count as f32) / (rows as f32);

		LayoutIter{
			rows: rows,
			columns_per_row: columns_per_row.trunc() as u16,
			row_interval_to_distribute_to: if columns_per_row.fract()==0.0{f32::NAN}else{1.0/columns_per_row.fract()},
			row: 0
		}
	}

	///Min and max number of columns for all rows
	pub fn columns(self) -> (u16,u16){
		let columns_per_row = {
			let f = (self.count as f32) / (self.rows() as f32);
			(f.trunc() as u16,f.fract())
		};

		(
			columns_per_row.0,
			columns_per_row.0 + if columns_per_row.1==0.0{0}else{1}
		)
	}
}

///Try to distribute as evenly as possible for the columns of each row based on the total number of rows
///Iterator for the number of columns for each row
#[derive(Copy,Clone,PartialEq)]
pub struct LayoutIter{
	pub rows: u16,
	row: u16,
	columns_per_row: u16,
	row_interval_to_distribute_to: f32
}
impl iter::Iterator for LayoutIter{
	///(row: u16,columns: u16)
	type Item = (u16,u16);

	fn nth(&mut self,row: usize) -> Option<<Self as iter::Iterator>::Item>{
		let row = row as u16;

		if row < self.rows{
			Some((
				row,

				if self.row_interval_to_distribute_to!=f32::NAN && ((row as f32) % self.row_interval_to_distribute_to).trunc() as u16 == 0{
					self.columns_per_row + 1
				}else{
					self.columns_per_row
				}
			))
		}else{
			None
		}
	}

	#[inline(always)]
	fn next(&mut self) -> Option<<Self as iter::Iterator>::Item>{
		let row = self.row as usize;
		let out = self.nth(row);
		self.row+= 1;
		out
	}

	#[inline(always)]
	fn size_hint(&self) -> (usize, Option<usize>){(self.rows as usize,Some(self.rows as usize))}

	#[inline(always)]
	fn count(self) -> usize{self.rows as usize}
}

#[derive(Copy,Clone,PartialEq)]
pub struct SizedLayoutIter{
	pub iter: LayoutIter,
	pub width: u32,
	pub height: u32,

	pub columns_left: u16
}

impl iter::Iterator for SizedLayoutIter{
	///(x: u32,y: u32,width: u32,height: u32)
	type Item = (u32,u32,u32,u32)
}

#[test]
fn test_views(){
	for count in 1..20{
		let views = RectangularViews{count: count,ratio: 1.0};
		println!("Views: {} ; Ratio: {}",views.count,views.ratio);
		let mut count2 = 0;
		for (row,columns) in views.layout(){
			println!("\t{}: {}",row,columns);
			count2+= columns;
		}
		println!("\tMatching: {}={}",count,count2);
	}
}
