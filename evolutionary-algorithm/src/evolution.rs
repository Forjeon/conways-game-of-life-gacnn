use super::{FitnessEvaluator, Matcher, Mutator, Recombinator, Reproducer, Selector};

pub struct Evolution<T> {
	generation: Vec<T>,
}

impl<T> Evolution<T> {
	pub fn new<Initializer: GenerationInitializer<T>>(population_size: usize) -> Result<Evolution<T>, ()> {
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

	pub fn evolve<Convergence, Crossover, Fitness, MateSelection, Mutation, Reproduction, Selection>(&mut self) -> usize
	where
		Convergence: EvolutionConvergenceChecker<T>,
		Crossover: Recombinator<T>,
		Fitness: FitnessEvaluator<T>,
		MateSelection: Matcher<T>,
		Mutation: Mutator<T>,
		Reproduction: Reproducer<T, Crossover, Mutation>,
		Selection: Selector<T, Fitness>,
	{
		let mut num_generations = 0usize;
		while !Convergence::is_converged(num_generations, &self.generation) {
			num_generations += 1;

			let progenitors = Selection::select(&self.generation);
			let mates = MateSelection::match_mates(&progenitors);
			self.generation = Reproduction::reproduce(&mates, self.generation.len());
		}
		num_generations
	}
}

pub trait EvolutionConvergenceChecker<T> {
	fn is_converged(generations: usize, solutions: &[T]) -> bool;
}

pub trait GenerationInitializer<T> {
	fn initialize(population_size: usize) -> Vec<T>;
}
