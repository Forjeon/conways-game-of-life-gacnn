pub trait Backprop {
	fn backpropagate(&mut self, loss: f64) -> f64;
}
