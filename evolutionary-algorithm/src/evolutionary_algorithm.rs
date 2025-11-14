use super::crossover::Crossover;
use super::fitness::Fitness;
use super::mutation::Mutation;
use super::selection::Selection;

pub trait EvolutionConvergence<T> {
	fn is_converged(generations: u64, solutions: &[T]) -> bool;
}

pub struct EvolutionaryAlgorithm<T> {
	generation: Vec<T>,
}

impl<T> EvolutionaryAlgorithm<T> {
	pub fn new(population_size: u64, initializer: impl GenerationInit<T>) -> EvolutionaryAlgorithm<T> {
		todo!()
	}

	pub fn new_from(f0: Vec<T>) -> EvolutionaryAlgorithm<T> {
		todo!()
	}

	pub fn solutions(&self) -> &[T] {
		todo!()
	}

	pub fn evolve(&mut self, convergence: impl EvolutionConvergence<T>, fitness: impl Fitness<T>, selection: impl Selection<T>, crossover: impl Crossover<T>, mutation: impl Mutation<T>) -> () {
		todo!()
	}
}

pub trait GenerationInit<T> {
	fn initialize(population_size: u64) -> Vec<T>;
}
