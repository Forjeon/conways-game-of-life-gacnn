use super::ActivationFunction;

pub trait Feedforward<Input, Activation: ActivationFunction, Output> {
	fn process(inputs: &[Input], bias: Option<Input>) -> Output;
}
