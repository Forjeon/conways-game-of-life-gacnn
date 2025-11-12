use rand::distr::{Bernoulli, Distribution, Uniform};
use rand::rngs::ThreadRng;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_AXIS_MAX: usize = BOARD_WIDTH - 1;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;

pub type ConwayBoard = [bool; BOARD_SIZE];

struct ConwayGame {
	generation: ConwayBoard
}

impl ConwayGame {
	pub fn new(rng: &mut ThreadRng, sparsity: f32) -> ConwayGame {
		let bernoulli_distr = Bernoulli::new(sparsity.into()).unwrap();

		let mut generation: ConwayBoard = [false; BOARD_SIZE];
		for cell in &mut generation {
			*cell = bernoulli_distr.sample(rng);
		}
		
		ConwayGame{ generation }
	}

	fn cell_coords_to_index(x: isize, y: isize) -> usize {
		let x = 0usize.wrapping_add_signed(x);
		let y = 0usize.wrapping_add_signed(y);
		(x % BOARD_WIDTH) + BOARD_WIDTH * (y % BOARD_WIDTH)
	}

	fn get_cell(&self, x: isize, y: isize) -> bool {
		self.generation[ConwayGame::cell_coords_to_index(x, y)]
	}

	fn count_live_neighbors(&self, x: isize, y: isize) -> u8 {
		let mut count = 0;
		for j in -1..=1 {
			for i in -1..=1 {
				count += match (i, j, self.get_cell(x.wrapping_add(i), y.wrapping_add(j))) {
					(0, 0, _) => 0,
					(.., true) => 1,
					_ => 0,
				};
			}
		}
		count
	}

	fn tick(&mut self) -> () {
		let mut successor: ConwayBoard = self.generation;

		for y in 0..BOARD_WIDTH {
			for x in 0..BOARD_WIDTH {
				let x = x.try_into().unwrap();
				let y = y.try_into().unwrap();
				successor[ConwayGame::cell_coords_to_index(x, y)] = match self.count_live_neighbors(x, y) {
					2 => self.get_cell(x, y),	// Persist
					3 => true,					// Persist or reproduce
					_ => false,					// Die
				};
			}
		}

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
	let mut instances = Vec::<(ConwayBoard, u32, ConwayBoard)>::new();
	let mut rng = rand::rng();
	let timestep_distr = Uniform::try_from(1..(max_timesteps + 1)).unwrap();

	for _ in 0..num_instances {
		let mut game = ConwayGame::new(&mut rng, sparsity);
		let initial_board = game.generation;
		let timestep = timestep_distr.sample(&mut rng);
		game.tick_by(timestep);
		instances.push((initial_board, timestep, game.generation));
	}

	instances
}


#[test]
fn test_get_cell() {
	let game0 = ConwayGame { generation: [
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
			assert!(!game0.get_cell(x.try_into().unwrap(), y.try_into().unwrap()));
		}
	}

	let game1 = ConwayGame { generation: [
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
	] };
	for y in 0..BOARD_WIDTH {
		for x in 0..BOARD_WIDTH {
			match x % 2 {
				0 => assert!(game1.get_cell(x.try_into().unwrap(), y.try_into().unwrap())),
				_ => assert!(!game1.get_cell(x.try_into().unwrap(), y.try_into().unwrap())),
			}
		}
	}

	let game2 = ConwayGame { generation: [
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
	] };
	for x in 0..BOARD_WIDTH {
		for y in 0..BOARD_WIDTH {
			assert!(game2.get_cell(x.try_into().unwrap(), y.try_into().unwrap()));
		}
	}
}

#[test]
fn test_get_cell_toroidal() {
	let game0 = ConwayGame { generation: [
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
		true, false, true, false, true, false, true, false,
	] };
	// Test (0, 0) toroidal neighbors
	assert!(!game0.get_cell(-1, -1));
	assert!(game0.get_cell(0, -1));
	assert!(!game0.get_cell(1, -1));
	assert!(!game0.get_cell(-1, 0));
	assert!(game0.get_cell(0, 0));
	assert!(!game0.get_cell(1, 0));
	assert!(!game0.get_cell(-1, 1));
	assert!(game0.get_cell(0, 1));
	assert!(!game0.get_cell(1, 1));
	// Test (7, 7) toroidal neighbors
	assert!(game0.get_cell(6, 6));
	assert!(!game0.get_cell(7, 6));
	assert!(game0.get_cell(8, 6));
	assert!(game0.get_cell(6, 7));
	assert!(!game0.get_cell(7, 7));
	assert!(game0.get_cell(8, 7));
	assert!(game0.get_cell(6, 8));
	assert!(!game0.get_cell(7, 8));
	assert!(game0.get_cell(8, 8));
	// Test extreme cases
	assert!(!game0.get_cell(-1384061901, -1384061901));	// (3, 3)
	assert!(game0.get_cell(-4, -4));					// (4, 4)
	assert!(game0.get_cell(1187311368, 1187311368));	// (0, 0)
	assert!(!game0.get_cell(23, 13));					// (7, 5)

	let game1 = ConwayGame { generation: [
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
		false, true, false, true, false, true, false, true,
	] };
	// Test (0, 0) toroidal neighbors
	assert!(game1.get_cell(-1, -1));
	assert!(!game1.get_cell(0, -1));
	assert!(game1.get_cell(1, -1));
	assert!(game1.get_cell(-1, 0));
	assert!(!game1.get_cell(0, 0));
	assert!(game1.get_cell(1, 0));
	assert!(game1.get_cell(-1, 1));
	assert!(!game1.get_cell(0, 1));
	assert!(game1.get_cell(1, 1));
	// Test (7, 7) toroidal neighbors
	assert!(!game1.get_cell(6, 6));
	assert!(game1.get_cell(7, 6));
	assert!(!game1.get_cell(8, 6));
	assert!(!game1.get_cell(6, 7));
	assert!(game1.get_cell(7, 7));
	assert!(!game1.get_cell(8, 7));
	assert!(!game1.get_cell(6, 8));
	assert!(game1.get_cell(7, 8));
	assert!(!game1.get_cell(8, 8));
	// Test extreme cases
	assert!(game1.get_cell(241777899, 14));		// (3, 4)
	assert!(!game1.get_cell(-126, 241777899));	// (6, 3)
}

#[test]
fn test_count_live_neighbors() {
	let game0 = ConwayGame { generation: [
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	for y in 1..(BOARD_WIDTH -1) {
		for x in 1..(BOARD_WIDTH -1) {
			assert!(game0.count_live_neighbors(x.try_into().unwrap(), y.try_into().unwrap()) == 0);
		}
	}

	let game1 = ConwayGame { generation: [
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
	] };
	for y in 1..BOARD_AXIS_MAX {
		for x in 1..BOARD_AXIS_MAX {
			assert!(game1.count_live_neighbors(x.try_into().unwrap(), y.try_into().unwrap()) == 8);
		}
	}

	let game2 = ConwayGame { generation: [
		false, false, false, true, false, true, false, false,
		false, true, false, false, false, false, false, false,
		false, false, false, true, false, true, false, false,
		true, true, false, false, false, false, false, false,
		false, true, false, false, false, false, false, false,
		false, false, false, true, true, true, false, false,
		false, false, false, true, true, true, false, false,
		false, false, false, true, true, true, false, false,
	] };
	assert!(game2.count_live_neighbors(1, 1) == 0);
	assert!(game2.count_live_neighbors(4, 1) == 4);
	assert!(game2.count_live_neighbors(6, 1) == 2);
	assert!(game2.count_live_neighbors(1, 4) == 2);
	assert!(game2.count_live_neighbors(1, 6) == 0);
	assert!(game2.count_live_neighbors(4, 6) == 8);
}

#[test]
fn test_count_live_neighbors_toroidal() {
	let game0 = ConwayGame { generation: [
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	for y in 0..BOARD_WIDTH {
		for x in 0..BOARD_WIDTH {
			assert!(game0.count_live_neighbors(x.try_into().unwrap(), y.try_into().unwrap()) == 0);
		}
	}

	let game1 = ConwayGame { generation: [
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
		true, true, true, true, true, true, true, true,
	] };
	for y in 0..BOARD_WIDTH {
		for x in 0..BOARD_WIDTH {
			assert!(game1.count_live_neighbors(x.try_into().unwrap(), y.try_into().unwrap()) == 8);
		}
	}

	let game2 = ConwayGame { generation: [
		true, false, false, true, false, true, false, true,
		false, true, false, false, false, false, false, false,
		false, false, false, true, false, true, false, false,
		true, true, false, false, false, false, false, false,
		false, true, false, false, false, false, false, false,
		false, false, false, true, true, true, false, false,
		true, false, false, true, true, true, false, true,
		false, false, false, true, true, true, false, true,
	] };
	assert!(game2.count_live_neighbors(7, 3) == 1);
	assert!(game2.count_live_neighbors(2, 7) == 3);
	assert!(game2.count_live_neighbors(0, 7) == 5);
}

#[test]
fn test_tick_toroidal() {
	let mut game0 = ConwayGame { generation: [
		true,  false, false, false, false, true, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, true, true, false, false, false,
		false, false, false, true, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, true, false, true, false,
	] };
	// Test lone cell dies
	assert!(game0.get_cell(0, 0));
	assert!(game0.count_live_neighbors(0, 0) == 0);
	// Test small group lives
	assert!(game0.get_cell(3, 4));
	assert!(game0.count_live_neighbors(3, 4) == 2);
	// Test medium group repopulates and dies
	assert!(!game0.get_cell(5, 7));
	assert!(game0.count_live_neighbors(5, 7) == 3);
	game0.tick();
	// Test lone cell dies
	assert!(!game0.get_cell(0, 0));
	assert!(game0.count_live_neighbors(0, 0) == 0);
	// Test small group lives
	assert!(game0.get_cell(3, 4));
	assert!(game0.count_live_neighbors(3, 4) == 3);
	// Test medium group repopulates and dies
	assert!(game0.get_cell(5, 7));
	assert!(game0.count_live_neighbors(5, 7) == 1);

	let mut game1 = ConwayGame { generation: [
		true, false, false, false, false, false, false, true,
		false, false, false, false, false, false, false, false,
		false, false, false, true, false, false, false, false,
		false, false, true, true, true, false, false, false,
		false, false, false, true, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		true, false, false, false, false, false, false, true,
	] };
	// Test medium group persists
	assert!(game1.get_cell(0, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7) && game1.get_cell(7, 7));
	// Test large group dies and persists
	assert!(game1.get_cell(3, 3));
	assert!(game1.count_live_neighbors(3, 3) == 4);
	game1.tick();
	// Test medium group persists
	assert!(game1.get_cell(0, 0) && game1.get_cell(7, 0) && game1.get_cell(0, 7) && game1.get_cell(7, 7));
	// Test large group dies and persists
	assert!(!game1.get_cell(3, 3));
	assert!(game1.count_live_neighbors(3, 3) == 8);
}

#[test]
fn test_tick_by_toroidal() {
	let mut game0 = ConwayGame { generation: [
		false, true,  false, false, false, false, false, false,
		false, false, true,  false, false, false, false, false,
		true,  true,  true,  false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
		false, false, false, false, false, false, false, false,
	] };
	assert!(game0.get_cell(1, 0) && game0.get_cell(2, 1) && game0.get_cell(0, 2) && game0.get_cell(1, 2) && game0.get_cell(2, 2));
	game0.tick_by(4);
	assert!(game0.get_cell(2, 1) && game0.get_cell(3, 2) && game0.get_cell(1, 3) && game0.get_cell(2, 3) && game0.get_cell(3, 3));
	game0.tick_by(28);
	assert!(game0.get_cell(1, 0) && game0.get_cell(2, 1) && game0.get_cell(0, 2) && game0.get_cell(1, 2) && game0.get_cell(2, 2));
	game0.tick_by(32);
	assert!(game0.get_cell(1, 0) && game0.get_cell(2, 1) && game0.get_cell(0, 2) && game0.get_cell(1, 2) && game0.get_cell(2, 2));
}
