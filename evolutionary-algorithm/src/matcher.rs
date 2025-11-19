pub trait Matcher<T> {
	fn match_mates(progenitors: &[T]) -> Vec<(T, T)>;
}
