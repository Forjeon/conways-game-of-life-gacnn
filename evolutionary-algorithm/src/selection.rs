pub trait Selection<T> {
	fn select(population: &[T], count: u64) -> &[T];
}
