pub mod activation_function;
pub mod backprop;
pub mod feedforward;

pub use activation_function::ActivationFunction;
pub use backprop::Backprop;
pub use feedforward::Feedforward;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
