pub trait Fitness<T> {
	fn evaluate(individual: &T) -> f64;
}
