// baseline.rs
use std::vec::Vec;
use std::collections::VecDeque;
use std::fmt;
use std::cmp;
use std::ptr;

pub struct CostField {
	width: usize,
	height: usize,
	data: Vec<u8>
}

// #[derive(Debug)]
pub struct IntegrationField {
	goal: (usize, usize),
	size: (usize, usize),
	data: Vec<u16>
}

pub struct FlowField {
	width: usize,
	height: usize,
	data: Vec<Direction>	
}

impl CostField {
	pub fn new(width: usize, height: usize) -> Self {
		assert!(width <= 1000 && height <= 1000);
		CostField {
			width: width,
			height: height,
			data: vec![1; width * height],
		}
	}

	#[inline]
	pub fn get(&self, x: usize, y: usize) -> u16 {
		self.data[self.width * y + x] as u16
	}

	#[inline]
	pub fn set(&mut self, x: usize, y: usize, cost: u8) {
		self.data[self.width * y + x] = cost;
	}
}

#[derive(Clone, Copy)]
pub struct TodoItem {
	x: u16,
	y: u16,
	pub cost: u16,
}

impl TodoItem {
	#[inline(always)]
	pub fn new(x: usize, y: usize, cost: u16) -> TodoItem {
		TodoItem {
			x: x as u16,
			y: y as u16,
			cost: cost,
		}
	}

	#[inline(always)]
	pub fn x(&self) -> usize {
		self.x as usize
	}

	#[inline(always)]
	pub fn y(&self) -> usize {
		self.y as usize
	}
}

impl fmt::Debug for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	write!(f, "<{}, {}|{}>", self.x, self.y, self.cost)
    }
}

impl IntegrationField {
	pub fn new(width: usize, height: usize) -> Self {
		let mut f = IntegrationField {
			size: (width, height),
			goal: (0, 0),
			data: vec![0; width * height],
		};
		f.reset();
		f
	}

	#[inline]
	pub fn width(&self) -> usize {
		self.size.0
	}

	#[inline]
	pub fn height(&self) -> usize {
		self.size.1
	}

	#[inline]
	pub fn get(&self, x: usize, y: usize) -> u16 {
		self.data[self.width() * y + x]
	}

	#[inline]
	pub fn get_bounding(&self, x: isize, y: isize) -> u16 {
		if x < 0 || x >= self.width() as isize ||
			y < 0 || y >= self.height() as isize
		{
			0xffffu16
		} else {
			self.get(x as usize, y as usize)
		}
	}

	#[inline]
	pub fn set(&mut self, x: usize, y: usize, cost: u16) {
		//println!("cost[{},{}] = {}", x, y, cost);
		let w = self.width();
		self.data[w * y + x] = cost;
	}

	#[inline(never)]
	pub fn calculate(&mut self, x: usize, y: usize, costs: &CostField) -> usize {
		self.reset();
		self.goal = (x, y);
		let mut queue = VecDeque::new();

		queue.push_back(TodoItem::new(x, y, 0));
//		self.set(x, y, 0);

		let mut neighbours = Vec::with_capacity(5);

		//let mut max_queue_len = 0usize;

		loop {
			//println!("{:?}", &self, queue.len());
			//println!("queue={:?}", queue.len());
			//max_queue_len = cmp::max(max_queue_len, queue.len());
			if let Some(todo) = queue.pop_front() {
				let curr_cost = self.get(todo.x(), todo.y());
				let new_cost = costs.get(todo.x(), todo.y()) + todo.cost;
				if new_cost < curr_cost {
					self.set(todo.x(), todo.y(), new_cost);
					self.set_neighbours(&mut neighbours, todo.x(), todo.y());
					for &(nx, ny) in neighbours.iter() {
						queue.push_back(TodoItem::new(nx, ny, new_cost));
					}
				}
			} else {
				break;
			}
		}
		//println!("max_queue_len={}", max_queue_len);
		queue.len() + 1
	}

	// about 800 us for 1000x1000 (2.5 GB/s)
	pub fn reset_safe(&mut self) {
		self.data.clear();
		let size = self.size.0 * self.size.1;
		self.data.reserve(size);
		for i in 0..size {
			self.data.push(0xffffu16);
		}
	}

	// about 67 us for 1000x1000 (29.8 GB/s, approx. 16 B/cycle)
	#[inline(never)]
	pub fn reset(&mut self) {
		self.data.clear();
		let size = self.size.0 * self.size.1;
		self.data.reserve(size);
		// same as setting all values to "0xffffu16"
		unsafe {
			self.data.set_len(size);
			let p = self.data.as_mut_ptr();
			ptr::write_bytes(p, 0xffu8, size);
		}
	}


	pub fn set_neighbours(&self, output: &mut Vec<(usize, usize)>, x: usize, y: usize) {
		output.clear();
		if x > 0 {
			output.push((x - 1, y));
		}
		if x < self.size.0 - 1 {
			output.push((x + 1, y));
		}
		if y > 0 {
			output.push((x, y - 1));
		}
		if y < self.size.1 - 1 {
			output.push((x, y + 1));
		}
		//println!("neighbours({}, {}) = {:?}", x, y, &output);
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Direction(u8);

impl Direction {
	pub fn none() -> Direction {
		Direction(0)
	}

	pub fn north() -> Direction {
		Direction(1)
	}

	pub fn northeast() -> Direction {
		Direction(2)
	}

	pub fn east() -> Direction {
		Direction(3)
	}

	pub fn southeast() -> Direction {
		Direction(4)
	}

	pub fn south() -> Direction {
		Direction(5)
	}

	pub fn southwest() -> Direction {
		Direction(6)
	}

	pub fn west() -> Direction {
		Direction(7)
	}

	pub fn northwest() -> Direction {
		Direction(8)
	}
}

static NORMAL_DIRECTIONS: &'static [u8] = &[
    //    n e s w
	0, // 0 0 0 0 -> -
	1, // 1 0 0 0 -> N
	3, // 0 1 0 0 -> E
	2, // 1 1 0 0 -> NE
	5, // 0 0 1 0 -> S
	0, // 1 0 1 0 -> -
	4, // 0 1 1 0 -> SE
	3, // 1 1 1 0 -> E
	7, // 0 0 0 1 -> W
	8, // 1 0 0 1 -> NW
	0, // 0 1 0 1 -> -
	1, // 1 1 0 1 -> N
	6, // 0 0 1 1 -> SW
	7, // 1 0 1 1 -> W
	5, // 0 1 1 1 -> S
	0, // 1 1 1 1 -> -
];

#[inline]
fn direction(p: u16, neighbours: &[u16]) -> Direction {
	let mut d = 0;
	let mut the_min = 0xffffu16;
	let mut i = 1;
	let mut min_i = 0;

	for &n in neighbours.iter() {
		if n < the_min {
			the_min = n;
			min_i = i;
		}
		i += 1;
	}

	if the_min < p {
		Direction(min_i)
	} else {
		Direction(0)
	}

	// if let Some(idx) = neighbours.iter().position(|&o| o < p) {
	// 	let 
	// 	assert!(idx < 9);
	// 	Direction(idx as u8 + 1)
	// } else {
	// 	Direction(0)
	// }
}

impl FlowField {
	pub fn from_integration_field(field: &IntegrationField) -> Self {
		let width = field.width();
		let height = field.height();

		let size = width * height;
		let mut data = vec![Direction::none(); size];

		for y in 0..height as isize {
			for x in 0..width as isize {
				let idx = y as usize * width + x as usize;
				data[idx] = direction(field.get_bounding(x, y),
					&[field.get_bounding(x, y - 1),     // N
					  field.get_bounding(x + 1, y - 1), // NE
					  field.get_bounding(x + 1, y),     // E
					  field.get_bounding(x + 1, y + 1), // SE
					  field.get_bounding(x, y + 1),     // S
					  field.get_bounding(x - 1, y + 1), // SW
					  field.get_bounding(x - 1, y),     // W
					  field.get_bounding(x - 1, y - 1)] // NW
					  );
			}
		}

		FlowField {
			width: width,
			height: height,
			data: data,
		}
	}
}

impl fmt::Debug for FlowField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for y in 0..self.height {
    		for x in 0..self.width {
    			match self.data[y * self.width + x] {
    				Direction(0) => write!(f, " ·")?,
    				Direction(1) => write!(f, " ↑")?,
    				Direction(2) => write!(f, " ↗")?,
    				Direction(3) => write!(f, " →")?,
    				Direction(4) => write!(f, " ↘")?,
    				Direction(5) => write!(f, " ↓")?,
    				Direction(6) => write!(f, " ↙")?,
    				Direction(7) => write!(f, " ←")?,
    				Direction(8) => write!(f, " ↖")?,
    				_ => write!(f, " ")?,
    			}
    		}
    		write!(f, "\n")?;
    	}
    	Ok(())
    }
}

impl fmt::Debug for IntegrationField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for y in 0..self.size.1 {
    		for x in 0..self.size.0 {
    			let n = self.get(x, y);
    			if n == 0xffffu16 {
    				write!(f, " xxxx")?;
    			} else {
	    			write!(f, " {:4}", n)?;
    			}
    		}
    		write!(f, "\n")?;
    	}
    	Ok(())
    }
}

impl fmt::Debug for CostField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for y in 0..self.height {
    		for x in 0..self.width {
    			let n = self.get(x, y);
    			write!(f, " {:3}", n)?;
    		}
    		write!(f, "\n")?;
    	}
    	Ok(())
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {

	use super::*;
	use test::Bencher;

	#[bench]
	fn bench_reset_100(b: &mut Bencher) {
    	let size = 100;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        b.iter(|| field.reset() );
    }

	#[bench]
	fn bench_reset_1000(b: &mut Bencher) {
    	let size = 1000;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        b.iter(|| field.reset() );
    }

	#[bench]
    fn bench_100_a(b: &mut Bencher) {
    	let size = 100;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        b.iter(|| field.calculate(3, 2, &costs) );
    }

	#[bench]
    fn bench_100_b(b: &mut Bencher) {
    	let size = 100;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        b.iter(|| field.calculate(35, 51, &costs) );
    }

	#[bench]
    fn bench_1000(b: &mut Bencher) {
    	let size = 1000;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        b.iter(|| field.calculate(3, 2, &costs) );
    }

    #[bench]
    fn bench_flow_100(b: &mut Bencher) {
    	let size = 100;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        field.calculate(3, 2, &costs);
        b.iter(|| FlowField::from_integration_field(&field) );
    }

    #[bench]
    fn bench_flow_1000(b: &mut Bencher) {
    	let size = 1000;
        let costs = CostField::new(size, size);
        let mut field = IntegrationField::new(size, size);
        field.calculate(350, 511, &costs);
        b.iter(|| FlowField::from_integration_field(&field) );
    }
}