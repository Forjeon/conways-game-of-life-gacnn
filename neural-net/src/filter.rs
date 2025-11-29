use super::activation_function::Identity;
use super::Backprop;
use super::Feedforward;

pub struct Filter {
	// TODO: hyperparameters, etc.
	// TODO: kernel size, dilation, stride, padding, depth (channels)
	// TODO: associated type for input and output based on hyperparameters (compile-time type checking for layers compatability)
}

// TODO: feedforward<f64, Identity, f64>, backprop

#[cfg(test)]
mod filter_tests {
	use super::*;
}
