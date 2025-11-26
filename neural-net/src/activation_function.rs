pub trait ActivationFunction {
	fn activate(activation: f64) -> f64;
}

pub struct Identity {}
impl ActivationFunction for Identity {
	fn activate(activation: f64) -> f64 { activation }
}

pub struct ReLU {}
impl ActivationFunction for ReLU {
	fn activate(activation: f64) -> f64 {
		activation.max(0.0)
	}
}

pub struct Sigmoid {}
impl ActivationFunction for Sigmoid {
	fn activate(activation: f64) -> f64 {
		1.0 / (1.0 + (-1.0 * activation).exp())
	}
}

pub struct TanH {}
impl ActivationFunction for TanH {
	fn activate(activation: f64) -> f64 {
		activation.tanh()
	}
}

#[cfg(test)]
mod activation_function_tests {
	use super::{ActivationFunction, ReLU, Sigmoid, TanH};

	#[test]
	fn test_relu() {
		assert_eq!(ReLU::activate(-8972834.3), 0.0);
		assert_eq!(ReLU::activate(-1.0), 0.0);
		assert_eq!(ReLU::activate(-0.74), 0.0);
		assert_eq!(ReLU::activate(0.0), 0.0);
		assert_eq!(ReLU::activate(0.4), 0.4);
		assert_eq!(ReLU::activate(1.0), 1.0);
		assert_eq!(ReLU::activate(4023.432), 4023.432);
	}

	#[test]
	fn test_sigmoid() {
		assert_eq!(Sigmoid::activate(-8972834.3), 0.0);
		assert_eq!(Sigmoid::activate(-1.0), 0.2689414213699951);
		assert_eq!(Sigmoid::activate(-0.74), 0.323004143761477);
		assert_eq!(Sigmoid::activate(0.0), 0.5);
		assert_eq!(Sigmoid::activate(0.4), 0.598687660112452);
		assert_eq!(Sigmoid::activate(1.0), 0.7310585786300049);
		assert_eq!(Sigmoid::activate(4023.432), 1.0);
	}

	#[test]
	fn test_tanh() {
		assert_eq!(TanH::activate(-8972834.3), -1.0);
		assert_eq!(TanH::activate(-1.0), -0.7615941559557649);
		assert_eq!(TanH::activate(-0.74), -0.6291451614140355);
		assert_eq!(TanH::activate(0.0), 0.0);
		assert_eq!(TanH::activate(0.4), 0.3799489622552249);
		assert_eq!(TanH::activate(1.0), 0.7615941559557649);
		assert_eq!(TanH::activate(4023.432), 1.0);
	}
}
