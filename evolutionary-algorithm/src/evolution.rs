use super::{FitnessEvaluator, Matcher, Mutator, Recombinator, Reproducer, Selector};

pub struct Evolution<T> {
	generation: Vec<T>,
}

impl<T> Evolution<T> {
	pub fn new<Initializer: GenerationInitializer<T>>(population_size: u64) -> Result<Evolution<T>, ()> {
		match population_size {
			0 => Err(()),
			count => Ok(Evolution { generation: Initializer::initialize(count) }),
		}
	}

	pub fn new_from(generation: Vec<T>) -> Evolution<T> {
		Evolution { generation }
	}

	pub fn solutions(&self) -> &[T] {
		&self.generation
	}

	pub fn evolve<Convergence, Crossover, Fitness, MateSelection, Mutation, Selection>(&mut self) -> u64
	where
		Convergence: EvolutionConvergenceChecker<T>,
		Crossover: Recombinator<T>,
		Fitness: FitnessEvaluator<T>,
		MateSelection: Matcher<T>,
		Mutation: Mutator<T>,
		Selection: Selector<T, Fitness>,
	{
		let mut num_generations = 0u64;
		while !Convergence::is_converged(num_generations, &self.generation) {
			num_generations += 1;

			let progenitors = Selection::select(&self.generation);
			let mates = MateSelection::match_mates(&progenitors);
			self.generation = mates.into_iter()
				.map(|(parent1, parent2)| {
					let mut individual = Crossover::recombine(&parent1, &parent2);
					Mutation::mutate(&mut individual);
					individual
				})
				.collect();
		}
		num_generations
	}
}

pub trait EvolutionConvergenceChecker<T> {
	fn is_converged(generations: u64, solutions: &[T]) -> bool;
}

pub trait GenerationInitializer<T> {
	fn initialize(population_size: u64) -> Vec<T>;
}
