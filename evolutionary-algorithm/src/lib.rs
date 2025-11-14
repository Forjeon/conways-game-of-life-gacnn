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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
