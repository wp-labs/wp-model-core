pub trait AsValueRef<T> {
    fn as_value_ref(&self) -> &T;
    fn as_value_mutref(&mut self) -> &mut T;
}
