use super::recombinator::Recombinator;
use super::fitness_evaluator::FitnessEvaluator;
use super::mate_selector::MateSelector;
use super::mutator::Mutator;
use super::progenitors_selector::ProgenitorsSelector;

pub trait EvolutionConvergenceChecker<T> {
	fn is_converged(generations: u64, solutions: &[T]) -> bool;
}

pub struct EvolutionaryAlgorithm<T> {
	generation: Vec<T>,
}

impl<T> EvolutionaryAlgorithm<T> {
	pub fn new<Initializer: GenerationInitializer<T>>(population_size: u64) -> Result<EvolutionaryAlgorithm<T>, ()> {
		match population_size {
			0 => Err(()),
			count => Ok(EvolutionaryAlgorithm { generation: Initializer::initialize(count) }),
		}
	}

	pub fn new_from(generation: Vec<T>) -> EvolutionaryAlgorithm<T> {
		EvolutionaryAlgorithm { generation }
	}

	pub fn solutions(&self) -> &[T] {
		&self.generation
	}

	pub fn evolve<Convergence, Crossover, Fitness, MateSelection, Mutation, Selection>(&mut self) -> u64
	where
		Convergence: EvolutionConvergenceChecker<T>,
		Crossover: Recombinator<T>,
		Fitness: FitnessEvaluator<T>,
		MateSelection: MateSelector<T>,
		Mutation: Mutator<T>,
		Selection: ProgenitorsSelector<T, Fitness>,
	{
		let mut num_generations = 0u64;
		while !Convergence::is_converged(num_generations, &self.generation) {
			num_generations += 1;

			let mut progenitors = Selection::select(&self.generation);
			let mates = MateSelection::select_mates(&mut progenitors);
			self.generation = mates.into_iter()
				.map(|(parent1, parent2)| {
					let mut individual = Crossover::recombine(parent1, parent2);
					Mutation::mutate(&mut individual);
					individual
				})
				.collect();
		}
		num_generations
	}
}

pub trait GenerationInitializer<T> {
	fn initialize(population_size: u64) -> Vec<T>;
}
