pub trait Initializer<T, U, F> {
    fn initialize(count: usize, f: F) -> T;
}

impl<T, U, F> Initializer<T, U, F> for T
where
    T: std::iter::FromIterator<U>,
    F: FnMut(usize) -> U,
{
    fn initialize(count: usize, f: F) -> T {
        (0..count).map(f).collect::<T>()
    }
}
