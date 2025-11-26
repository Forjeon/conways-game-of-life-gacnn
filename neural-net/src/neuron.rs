use super::ActivationFunction;
use super::Backprop;
use super::Feedforward;

pub struct Neuron<Activation: ActivationFunction> {
	delete_me: Activation,
	// TODO: hyperparameters, etc.
}

// TODO: feedforward, backprop

#[cfg(test)]
mod neuron_tests {
	use super::*;
}
