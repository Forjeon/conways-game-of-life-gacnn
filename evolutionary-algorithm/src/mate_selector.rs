pub trait MateSelector<T> {
	fn select_mates(progenitors: &mut [T]) -> Vec<(&T, &T)>;
}
