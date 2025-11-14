pub trait Recombinator<T> {
	fn recombine(parent1: &T, parent2: &T) -> T;
}
