pub trait Mutator<T> {
	fn mutate(individual: &mut T) -> ();
}
