use super::{Mutator, Recombinator};

pub trait Reproducer<T, Crossover: Recombinator<T>, Mutation: Mutator<T>> {
	fn reproduce(mates: &[(T, T)], population_size: usize) -> Vec<T>;
}
