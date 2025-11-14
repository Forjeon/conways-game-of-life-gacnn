use super::fitness_evaluator::FitnessEvaluator;

pub trait ProgenitorsSelector<T, Fitness: FitnessEvaluator<T>> {
	fn select(population: &[T]) -> Vec<T>;
}
