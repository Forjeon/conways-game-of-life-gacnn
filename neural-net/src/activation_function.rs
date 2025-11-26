pub trait ActivationFunction {
	fn activate(activation: f64) -> f64;
}

pub struct Identity {}
impl ActivationFunction for Identity {
	fn activate(activation: f64) -> f64 { activation }
}

// ReLU, Sigmoid, Softmax, TanH
