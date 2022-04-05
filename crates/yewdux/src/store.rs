//! Unique state shared application-wide
pub use yewdux_macros::Store;

/// Globally shared state.
pub trait Store: Clone + PartialEq + 'static {
    /// Initialize this store.
    fn new() -> Self;

    /// Called after state has changed.
    fn changed(&mut self) {}
}

/// A type that can change state.
pub trait Reducer<S> {
    /// Mutate state.
    fn apply(&self, state: &mut S);
}
