pub trait Backprop {
	fn backpropagate(loss: f64) -> f64;
}
