pub trait Backprop {
	fn backpropagate(&mut self, learning_rate: f64, loss: f64) -> f64;
}
