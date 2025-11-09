#![allow(unused)]//FIXME:DEL
use rand::distr::{Bernoulli, Distribution};

const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

type ConwayBoard = [bool; BOARD_SIZE];

struct ConwayGame {
	generation: ConwayBoard
}

// TODO: scrap functions and start from scratch with sufficient unit testing (work on non-toroidal implementation first)
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

	fn get_live_neighbours(&self, x: usize, y: usize) -> u8 {
		todo!()
	}

	fn tick(&mut self) -> () {
		todo!()
	}

	pub fn tick_by(&mut self, timestep: u32) -> () {
		todo!()
	}
}

// Returns a vector of instances, each being a tuple of initial board, timestep, and target board
pub fn run(num_instances: u32, max_timesteps: u32, sparsity: f32) -> Vec<(ConwayBoard, u32, ConwayBoard)> {
	let mut board = ConwayGame::new(sparsity);
	board.tick_by(5);
	todo!()
}


#[test]
fn test_get_cell() {
	let mut game0 = ConwayGame { generation: [
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	for x in 0..BOARD_WIDTH {
		for y in 0..BOARD_WIDTH {
			assert!(!game0.get_cell(x, y));
		}
	}
}

/*
#[test]
fn test_get_live_neighbours() {
	todo!()
}

#[test]
fn test_tick_die() {
	let mut game0 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	assert!(game0.get_cell(0, 0));
	game0.tick_by(1);
	assert!(!game0.get_cell(0, 0));
	game0.tick_by(1000);
	assert!(!game0.get_cell(0, 0));

	let mut game1 = ConwayGame { generation: [
		true,  true,  false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, true,
	] };
	assert!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7));
	game1.tick_by(1);
	assert!(!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7)));
	game1.tick_by(12345);
	assert!(!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7)));
}

#[test]
fn test_tick_live() {
	let mut game1 = ConwayGame { generation: [
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, true,  false, false,
		false, false, false, false, true,  true,  false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	game1.tick_by(1);
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	game1.tick_by(42);
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	let mut game0 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, false,
	] };
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));
	game0.tick_by(1);
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));
	game0.tick_by(643253);
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));


	let mut game2 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, true,
	] };
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
	game2.tick_by(1);
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
	game2.tick_by(1234567890);
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
}

#[test]
fn test_tick_by_die() {
	let mut game0 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	assert!(game0.get_cell(0, 0));
	game0.tick_by(1);
	assert!(!game0.get_cell(0, 0));
	game0.tick_by(1000);
	assert!(!game0.get_cell(0, 0));

	let mut game1 = ConwayGame { generation: [
		true,  true,  false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, true,
	] };
	assert!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7));
	game1.tick_by(1);
	assert!(!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7)));
	game1.tick_by(12345);
	assert!(!(game1.get_cell(0, 0) && game1.get_cell(1, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7)));
}

#[test]
fn test_tick_by_live() {
	let mut game1 = ConwayGame { generation: [
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, true,  false, false,
		false, false, false, false, true,  true,  false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	game1.tick_by(1);
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	game1.tick_by(42);
	assert!(game1.get_cell(5, 1) && game1.get_cell(4, 2) && game1.get_cell(5, 2));
	let mut game0 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, false,
	] };
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));
	game0.tick_by(1);
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));
	game0.tick_by(643253);
	assert!(game0.get_cell(0, 0) && game0.get_cell(7, 0) && game0.get_cell(0, 7));


	let mut game2 = ConwayGame { generation: [
		true,  false, false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true,  false, false, false, false, false, false, true,
	] };
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
	game2.tick_by(1);
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
	game2.tick_by(1234567890);
	assert!(game2.get_cell(0, 0) && game2.get_cell(7, 0) && game2.get_cell(0, 7) && game2.get_cell(7, 7));
}
*/
