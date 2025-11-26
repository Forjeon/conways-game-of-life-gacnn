pub trait Loss<T> {
	fn calculate_loss(predictions: &[T], labels: &[T]) -> T;
}

pub struct LogLoss {}
impl<T> Loss<T> for LogLoss {
	fn calculate_loss(predictions: &[T], labels: &[T]) -> T {
		// sum from i=1 to n ( y-(true,i) * log(y-(pred,i)) )
		todo!()
	}
}

#[cfg(test)]
mod loss_tests {
	use super::{Loss, LogLoss};

	#[test]
	fn test_log_loss() {
		todo!()
	}
}
