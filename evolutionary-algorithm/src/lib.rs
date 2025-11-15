pub mod evolution;
pub mod fitness_evaluator;
pub mod matcher;
pub mod mutator;
pub mod reproducer;
pub mod recombinator;
pub mod selector;

pub use evolution::{Evolution, EvolutionConvergenceChecker, GenerationInitializer};
pub use fitness_evaluator::FitnessEvaluator;
pub use matcher::Matcher;
pub use mutator::Mutator;
pub use reproducer::Reproducer;
pub use recombinator::Recombinator;
pub use selector::Selector;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    //use super::evolution::Evolution;
	//use super::fitness_evaluator::FitnessEvaluator;
	//use super::matcher::Matcher;
	//use super::mutator::Mutator;
	//use super::recombinator::Recombinator;
	//use super::selector::Selector;
	use super::*;

    #[test]
	fn basic_ea() {
		struct BasicTest{}

		impl EvolutionConvergenceChecker<i32> for BasicTest{
			fn is_converged(generations: usize, _solutions: &[i32]) -> bool {
				generations >= 1
			}
		}

		impl FitnessEvaluator<i32> for BasicTest{
			fn evaluate(individual: &i32) -> f64 {
				(individual.abs() as f64) / 100f64
			}
		}

		impl Matcher<i32> for BasicTest {
			fn match_mates(progenitors: &[i32]) -> Vec<(i32, i32)> {
				let mut progenitors = progenitors.to_owned();
				progenitors.sort();
				let (left, right) = progenitors.split_at(progenitors.len() / 2);
				left.to_owned().into_iter().zip(right.to_owned()).collect()
			}
		}

		impl Mutator<i32> for BasicTest {
			fn mutate(individual: &mut i32) -> () {
				*individual += 1i32;
			}
		}

		impl Recombinator<i32> for BasicTest{
			fn recombine(parent1: &i32, parent2: &i32) -> i32 {
				parent1 + parent2
			}
		}

		impl Reproducer<i32, BasicTest, BasicTest> for BasicTest {
			fn reproduce(mates: &[(i32, i32)], population_size: usize) -> Vec<i32> {
				let mut successors = vec![];
				let mut mate_index = 0;
				while successors.len() < population_size.try_into().unwrap() {
					let mut offspring = BasicTest::recombine(&mates[mate_index].0, &mates[mate_index].1);
					BasicTest::mutate(&mut offspring);
					successors.push(offspring);
					mate_index += 1;
				}
				successors
			}
		}

		impl Selector<i32, BasicTest> for BasicTest {
			fn select(population: &[i32]) -> Vec<i32> {
				population.split_at(population.len() / 2).0.to_vec()
			}
		}

		let mut ea = Evolution::<i32>::new_from(vec![-3, -2, -1, 0, 1, 2, 3]);
		assert_eq!(ea.solutions(), vec![-3, -2, -1, 0, 1, 2, 3]);
		assert_eq!(ea.evolve::<BasicTest, BasicTest, BasicTest, BasicTest, BasicTest, BasicTest, BasicTest>(), 1);
		assert_eq!(ea.solutions(), vec![-3, -3, -3, -3, -3, -3, -3]);
	}
}
