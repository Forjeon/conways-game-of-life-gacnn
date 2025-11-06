use rand::distr::{Bernoulli, Distribution};

const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

type ConwayBoard = [bool; BOARD_SIZE];

struct ConwayGame {
	generation: ConwayBoard
}

impl ConwayGame {
	pub fn new(sparsity: f32) -> ConwayGame {
		let bernoulli_distr = Bernoulli::new(sparsity.into()).unwrap();
		let mut rng = rand::rng();

		let mut generation: ConwayBoard = [false; BOARD_SIZE];
		for cell in &mut generation {
			*cell = bernoulli_distr.sample(&mut rng);
		}
		
		ConwayGame{ generation }
	}

	fn get_cell(&self, x: usize, y: usize) -> bool {
		todo!()
	}

	fn tick(&mut self) -> () {
		let mut successor = self.generation;

		// Neighbour: any of the eight adjacent cells surrounding a given cell
		// Kill cells with less than two or more than three living neighbours (eight surrounding cells)
		todo!();

		// Reproduce cells with exactly three living neighbours
		todo!();

		self.generation = successor;
	}

	pub fn tick_by(&mut self, timestep: u32) -> () {
		for _ in 0..timestep {
			self.tick();
		}
	}
}

// Returns a vector of instances, each being a tuple of initial board, timestep, and target board
pub fn run(num_instances: u32, max_timesteps: u32, sparsity: f32) -> Vec<(ConwayBoard, u32, ConwayBoard)> {
	let mut board = ConwayGame::new(sparsity);
	board.tick_by(5);
	todo!()
}
