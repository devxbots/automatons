use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, Hasher};

type AnyMap = HashMap<TypeId, Box<dyn Any + Send + Sync>, BuildHasherDefault<IdHasher>>;

/// In-memory state for tasks
///
/// Steps in a task can share information with each other by putting it into a shared state. The
/// state leverages Rust's type system and uses type ids to index information. In combination with
/// the newtype pattern, this creates a flexible but still strongly typed store.
///
/// # Example
///
/// ```rust
/// use automatons::State;
///
/// let mut state = State::new();
/// state.insert("example");
///
/// assert_eq!(Some(&"example"), state.get::<&str>());
/// ```
///
/// # Acknowledgements
///
/// The implementation for this type-based map is inspired by the `Extensions` store in the
/// [`http`](https://github.com/hyperium/http) crate.
#[derive(Debug, Default)]
pub struct State {
    store: Box<AnyMap>,
}

impl State {
    /// Initializes an empty state.
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn new() -> Self {
        Self {
            store: Box::new(HashMap::default()),
        }
    }

    /// Inserts the given value into the state.
    ///
    /// The given value is added to the state's internal store. If the store already contains data
    /// with the same type, it will be overwritten by the new value.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use automatons::State;
    /// #
    /// let mut state = State::new();
    ///
    /// // Data is indexed by its type, in this case u32
    /// state.insert(0u32);
    /// assert_eq!(Some(&0u32), state.get::<u32>());
    ///
    /// // Adding another u32 overwrites the original value
    /// state.insert(1u32);
    /// assert_eq!(Some(&1u32), state.get::<u32>());
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn insert<T: Send + Sync + 'static>(&mut self, val: T) -> Option<T>
    where
        T: Debug,
    {
        self.store
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| {
                (boxed as Box<dyn Any + 'static>)
                    .downcast()
                    .ok()
                    .map(|boxed| *boxed)
            })
    }

    /// Returns an option with the requested data type
    ///
    /// The state uses type ids to index data, which can be leveraged when looking up data in the
    /// store. In most cases, the compiler will be able to infer the correct data type. But the type
    /// can also by explicitly specified using the turbofish operator or a type hint.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use automatons::State;
    /// #
    /// # fn example() -> Option<u32> {
    /// let mut state = State::new();
    /// state.insert(2u32);
    ///
    /// let a: u32 = 3;
    /// let b = state.get()?;
    ///
    /// assert_eq!(6, a * b);
    /// # None
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.store
            .get(&TypeId::of::<T>())
            .and_then(|boxed| (&**boxed as &(dyn Any + 'static)).downcast_ref())
    }

    /// Returns an option with a mutable reference for the requested data type
    ///
    /// The state uses type ids to index data, which can be leveraged when looking up data in the
    /// store. In most cases, the compiler will be able to infer the correct data type. But the type
    /// can also by explicitly specified using the turbofish operator or a type hint.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use automatons::State;
    /// #
    /// # fn example() -> Option<u32> {
    /// let mut state = State::new();
    /// state.insert(String::from("Hello"));
    ///
    /// let mut string = state.get_mut::<String>()?;
    /// string.push_str(", World!");
    ///
    /// assert_eq!(&String::from("Hello, World!"), state.get::<String>()?);
    /// # None
    /// # }
    /// ```
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.store
            .as_mut()
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| (&mut **boxed as &mut (dyn Any + 'static)).downcast_mut())
    }
}

#[derive(Debug, Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }
}

#[cfg(test)]
mod tests {
    use super::State;

    #[test]
    fn state_stores_and_returns_value() {
        let mut state = State::new();

        state.insert(64u32);

        assert_eq!(Some(&64), state.get::<u32>());
    }

    #[test]
    fn state_returns_none_when_value_is_missing() {
        let mut state = State::new();

        state.insert(64u32);

        assert_eq!(None, state.get::<i32>());
    }

    #[test]
    fn trait_send() {
        fn assert_send<T: Send>() {}
        assert_send::<State>();
    }

    #[test]
    fn trait_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<State>();
    }
}
