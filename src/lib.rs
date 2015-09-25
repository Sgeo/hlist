pub struct Nil;
pub struct Cons<H, T>(pub H, pub T);

#[allow(dead_code)]
pub enum Here {}
#[allow(dead_code)]
pub struct There<T>(std::marker::PhantomData<T>);

impl Nil {
    pub fn push<N>(self, item: N) -> Cons<N, Self> {
        Cons(item, self)
    }
}

impl<H, T> Cons<H, T> {
    pub fn push<N>(self, item: N) -> Cons<N, Self> {
        Cons(item, self)
    }
}

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

impl<Head, T, Tail, TailI> Find<T, There<TailI>> for Cons<Head, Tail>
    where Tail: Find<T, TailI> {
    fn get(&self) -> &T {
        self.1.get()
    }
    fn get_mut(&mut self) -> &mut T {
        self.1.get_mut()
    }
}

#[test]
fn it_works() {
}
