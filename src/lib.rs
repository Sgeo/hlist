
#![warn(missing_docs)]

//! This crate provides types `Nil` and `Cons<H, T>`, which together allow for creating lists consisting of multiple types.
//! The types in the list are present in the type of the list, so that `Cons<i32, Cons<i64, Nil>>` contains an `i32` and an `i64`.
//! If type `T` is present exactly once in an `HList`, it is usually possible to allow the compiler to find the `T` in the `HList` by using the `Find` trait.

/// The empty `HList`.
pub struct Nil;

/// An `HList` with `H` at position 0, and `T` as the rest of the list.
pub struct Cons<H, T>(pub H, pub T);

/// A marker trait that `Nil` and `Cons<H, T>` satisfies.
/// Not currently used to enforce proper hlists, although this may change.
/// Provides the `push()` method
pub trait HList: Sized {
    /// Consumes the `HList`, and returns a new HList with `item` at the beginning.
    fn push<N>(self, item: N) -> Cons<N, Self> {
        Cons(item, self)
    }
}

impl HList for Nil {}
impl<H, T> HList for Cons<H, T> {}


/// Used as an index into an `HList`.
///
/// `Here` is 0, pointing to the head of the HList.
///
/// Users should normally allow type inference to create this type
#[allow(dead_code)]
pub enum Here {}


/// Used as an index into an `HList`.
///
/// `There<T>` is 1 + `T`.
///
/// Users should normally allow type inference to create this type.
#[allow(dead_code)]
pub struct There<T>(std::marker::PhantomData<T>);


/// `Find<T, I>` is implemented for an `HList` if index `I` of the `HList` is a `T`
///
/// Rust's type inferencer can often produce a correct `I`
/// if there is exactly one `T` in the `HList`.
///
/// ```rust
/// use hlist::{HList, Nil, Find};
///
/// // The type of list is Cons<i64, Cons<i32, Nil>>
/// let list = Nil.push(0i32).push(1i64);
///
/// // Here list satisfies the trait Find<i64, Here>.
/// // The compiler infers the second type parameter.
/// let a: i64 = *list.get();
/// assert!(a == 1);
///
/// // Here list satisfies the trait Find<i32, There<Here>>.
/// let b: i32 = *list.get(); 
/// assert!(b == 0);
/// ```
///
/// Functions that need to look up values of a type in an HList given to them should get the index from the call site:
///
/// ```rust
/// use hlist::{HList, Nil, Find};
///
/// fn foo<I, L: Find<i32, I>>(list: &L) -> i32 {
///     *list.get()
/// }
/// let list = Nil.push("foo").push(5i32).push("bar");
/// assert!(foo(&list) == 5);
/// ```
/// 
/// When `foo()` is called, the compiler figures out the appropriate value for `I`.
pub trait Find<T, I> {
    /// Retrieves a `&T`.
    ///
    /// Allows for type inferencing to act like type-directed search.
    ///
    /// ```rust
    /// use hlist::{HList, Nil, Find};
    ///
    /// let list = Nil.push(0i32).push(1i64);
    /// let a: i64 = *list.get();
    /// assert!(a == 1);
    fn get(&self) -> &T;
    
    /// Retrieves a `&mut T`.
    ///
    /// Allows for type inferencing to act like type-directed search.
    ///
    /// ```rust
    /// use hlist::{HList, Nil, Find};
    ///
    /// let mut list = Nil.push(0i32).push(1i64);
    /// *list.get_mut() = 5i32;
    /// let a: i32 = *list.get();
    /// assert!(a == 5);
    fn get_mut(&mut self) -> &mut T;
}

impl<T, Tail> Find<T, Here> for Cons<T, Tail> {
    fn get(&self) -> &T {
        &self.0
    }
    fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<Head, T, Tail, TailIndex> Find<T, There<TailIndex>> for Cons<Head, Tail>
    where Tail: Find<T, TailIndex> {
    fn get(&self) -> &T {
        self.1.get()
    }
    fn get_mut(&mut self) -> &mut T {
        self.1.get_mut()
    }
}

#[test]
fn test_get() {
    let list = Nil.push(5i32).push("Foo");
    let a: i32 = *list.get();
    let b: &str = *list.get();
    assert!(a == 5i32);
    assert!(b == "Foo");
}

#[test]
fn test_get_mut() {
    let mut list = Nil.push(5i32).push("Foo");
    *list.get_mut() = 6i32;
    *list.get_mut() = "Bar";
    let a: i32 = *list.get();
    let b: &str = *list.get();
    assert!(a == 6i32);
    assert!(b == "Bar");
}

#[test]
fn test_index_as_type_parameter() {
    fn foo<I, L: Find<i32, I>>(list: &L) -> i32 {
        *list.get()
    }
    let list = Nil.push("foo").push(5i32).push("bar");
    assert!(foo(&list) == 5);
}
