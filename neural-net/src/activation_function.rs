pub trait ActivationFunction {
	fn activate(activation: f64) -> f64;
}

pub struct Identity {}
impl ActivationFunction for Identity {
	fn activate(activation: f64) -> f64 { activation }
}

// TODO: ReLU, Sigmoid, Softmax, TanH

#[cfg(test)]
mod activation_function_tests {
	use super::*;
}
