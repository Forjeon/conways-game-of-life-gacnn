pub trait ActivationFunction {
	fn activate(activation: f64) -> f64;
}

pub struct Identity {}
impl ActivationFunction for Identity {
	fn activate(activation: f64) -> f64 { activation }
}

// TODO: ReLU, Sigmoid, TanH

#[cfg(test)]
mod activation_function_tests {
	fn test_relu() {
		assert_eq!(ReLU::activate(-8972834.3), 0);
		assert_eq!(ReLU::activate(-1), 0);
		assert_eq!(ReLU::activate(-0.74), 0);
		assert_eq!(ReLU::activate(0), 0);
		assert_eq!(ReLU::activate(0.4), 0.4);
		assert_eq!(ReLU::activate(1), 1);
		assert_eq!(ReLU::activate(4023.432), 4023.432);
	}

	fn test_sigmoid() {
		assert_eq!(Sigmoid::activate(-8972834.3), 0);
		assert_eq!(Sigmoid::activate(-1), 0.2689414);
		assert_eq!(Sigmoid::activate(-0.74), 0.3230041);
		assert_eq!(Sigmoid::activate(0), 0.5);
		assert_eq!(Sigmoid::activate(0.4), 0.5986877);
		assert_eq!(Sigmoid::activate(1), 0.7310586);
		assert_eq!(Sigmoid::activate(4023.432), 1);
	}

	fn test_tanh() {
		assert_eq!(TanH::activate(-8972834.3), -1);
		assert_eq!(TanH::activate(-1), -0.7615941559557649);
		assert_eq!(TanH::activate(-0.74), -0.6291451614140355);
		assert_eq!(TanH::activate(0), 0);
		assert_eq!(TanH::activate(0.4), 0.3799489622552249);
		assert_eq!(TanH::activate(1), 0.7615941559557649);
		assert_eq!(TanH::activate(4023.432), 1);
	}
}
