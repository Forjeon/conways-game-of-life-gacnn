use super::ActivationFunction;

pub trait Feedforward<Input, Activation: ActivationFunction, Output> {
	fn process(&self, inputs: &[Input], bias: Option<Input>) -> Output;
}
