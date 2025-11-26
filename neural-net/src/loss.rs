pub trait Loss<T> {
	fn calculate_loss(predictions: &[T], labels: &[T]) -> T;
}

// TODO: log loss
