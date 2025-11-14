pub mod recombinator;
pub mod evolutionary_algorithm;
pub mod fitness_evaluator;
pub mod mate_selector;
pub mod mutator;
pub mod progenitors_selector;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn basic_ea() {
		struct BasicTest{}

		impl EvolutionConvergenceChecker<i32> for BasicTest{
			fn is_converged(generations: u64, solutions: &[i32]) -> bool {
				generations >= 10
			}
		}

		impl Recombinator<i32> for BasicTest{
			fn recombine(parent1: &i32, parent2: &i32) -> i32 {
				parent1 + parent2
			}
		}

		impl FitnessEvaluator<i32> for BasicTest{
			fn evaluate(individual: &i32) -> f64 {
				(individual.abs() as f64) / 100f64
			}
		}

		impl MateSelector<i32> for BasicTest {
			fn select_mates(progenitors: &mut [i32]) -> Vec<(&i32, &i32)> {
				progenitors.sort();
				let (left, right) = progenitors.split(progenitors.len() / 2);
				left.zip(right).collect()
			}
		}

		//impl Mutator<i32> for
		// Recombinator, FitnessEvaluator, MateSelector, Mutator, ProgenitorsSelector
		let mut ea = EvolutionaryAlgorithm::<i32>::new_from(vec![-3, -2, -1, 0, 1, 2, 3]);
		assert!(ea.solutions == vec![-3, -2, -1, 0, 1, 2, 3]);
		//ea.evolve<
	}
}
