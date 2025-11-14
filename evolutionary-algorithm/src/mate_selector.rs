use std::iter::Zip;
use std::slice::Iter;

pub trait MateSelector<T> {
	fn select_mates(progenitors: &[T]) -> Zip<Iter<T>, Iter<T>>;
}
