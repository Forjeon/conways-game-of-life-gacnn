use rand::distr::{Bernoulli, Distribution};

const BOARD_WIDTH: usize = 8;

type ConwayBoard = [bool; BOARD_WIDTH * BOARD_WIDTH];

// Returns a vector of instances, each being a tuple of initial board, timestep, and target board
pub fn run(num_instances: u32, max_timesteps: u32, sparsity: f32) -> Vec<(ConwayBoard, u32, ConwayBoard)> {
	// TODO: separate into functions for generating a board and advancing a board
	let bernoulli_distr = Bernoulli::new(sparsity.into()).unwrap();
	let mut rng = rand::rng();

	let mut board: [bool; BOARD_WIDTH * BOARD_WIDTH] = [false; BOARD_WIDTH * BOARD_WIDTH];
	for cell in &mut board {
		*cell = bernoulli_distr.sample(&mut rng);
	}
	todo!();
}
