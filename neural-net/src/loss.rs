pub trait Loss<T> {
	// TODO: improve compile-time safety of this function how? (predictions and labels must have the same length, which length must be nonzero)
	fn calculate_loss(predictions: &[T], labels: &[T]) -> T;
}

// TODO:FIXME: use BCE, CCE, or a different loss altogether?
pub struct LogLoss {}
impl<f64> Loss<f64> for LogLoss {
	fn calculate_loss(predictions: &[f64], labels: &[f64]) -> f64 {
		// TODO: panic! if lengths of predictions and labels are not equal and nonzero, or if any item in predictions or labels is outside of (0.0, 1.0]
		// sum from i=1 to n ( y-(true,i) * log(y-(pred,i)) )
		todo!()
	}
}

#[cfg(test)]
mod loss_tests {
	use super::{Loss, LogLoss};

	#[test]
	#[should_panic]
	fn log_loss_bad_pred_len() {
		LogLoss::calculate_loss([], [1.0, 0.0]);
	}

	#[test]
	#[should_panic]
	fn log_loss_bad_labels_len() {
		LogLoss::calculate_loss([0.12, 0.432], []);
	}

	#[test]
	#[should_panic]
	fn log_loss_bad_lens() {
		LogLoss::calculate_loss([], []);
	}

	#[test]
	#[should_panic]
	fn log_loss_inequal_lens() {
		LogLoss::calculate_loss([0.745], [0.0, 1.0, 0.0]);
	}

	#[test]
	#[should_panic]
	fn log_loss_bad_pred_item() {
		LogLoss::calculate_loss([0.234, 0.0, 1.0], [1.0, 0.0, 1.0]);
	}

	#[test]
	fn log_loss() {
		assert_eq!(LogLoss::calculate_loss([1.0, 0.0, 1.0, 0.0], [1.0, 0.0, 1.0, 0.0]), 0.0);
		assert_eq!(LogLoss::calculate_loss([0.0, 1.0, 0.0, 1.0], [1.0, 0.0, 1.0, 0.0]), 
	}
}
