
/// The empty `HList`.
pub struct Nil;

/// An `HList` with `H` at position 0, and `T` as the rest of the list.
pub struct Cons<H, T>(pub H, pub T);

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
/// Users should normally allow type inference to deal with this type
#[allow(dead_code)]
pub enum Here {}


/// Used as an index into an `HList`.
///
/// `There<T>` is 1 + `T`.
///
/// Users should normally allow type inference to deal with this type.
#[allow(dead_code)]
pub struct There<T>(std::marker::PhantomData<T>);



pub trait Find<T, I> {
    fn get(&self) -> &T;
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
