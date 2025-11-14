pub trait FitnessEvaluator<T> {
	fn evaluate(individual: &T) -> f64;
}
