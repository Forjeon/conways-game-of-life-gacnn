pub trait Mutation<T> {
	fn mutate(individual: &mut T) -> ();
}
