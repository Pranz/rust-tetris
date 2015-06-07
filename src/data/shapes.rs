

pub type Block = [[Bool; 4]; 4]

const square_block : Block = [
		[true, true, false, false],
		[true, true, false, false],
		[false, false, false, false],
		[false, false, false, false]
	];

const l_block : Block = [
	 [true, false, false, false],
	 [true, false, false, false],
	 [true, true, false, false],
	 [false, false, false, false]
]

const l_bloc k mirrored : Block [
	[false, true, false, false],
	[false, true, false, false],
	[true, true, false, false],
	[false, false, false, false]
]

pub fn block_intersects()