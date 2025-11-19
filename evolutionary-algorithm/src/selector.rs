use super::FitnessEvaluator;

pub trait Selector<T, Fitness: FitnessEvaluator<T>> {
	fn select(population: &[T]) -> Vec<T>;
}
