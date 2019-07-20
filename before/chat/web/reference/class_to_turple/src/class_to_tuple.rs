fn main() {
    // Example with some simplifications
    // Note that there is no extra allocation
    let num_pair: (&str, &str) = "flex center"
        .split_whitespace()
        .try_collect()
        .expect("a two-tuple of &str");
    println!("{:#?}", &num_pair);
    assert_eq!(num_pair, ("flex", "center",));
}

trait TryCollect<T> {
    fn try_collect(&mut self) -> Option<T>;
}

macro_rules! impl_try_collect_tuple {
    () => { };
    ($A:ident $($I:ident)*) => {
        impl_try_collect_tuple!($($I)*);

        impl<$A: Iterator> TryCollect<($A::Item, $($I::Item),*)> for $A {
            fn try_collect(&mut self) -> Option<($A::Item, $($I::Item),*)> {
                let r = (try_opt!(self.next()),
                         // hack: we need to use $I in the expasion
                         $({ let a: $I::Item = try_opt!(self.next()); a}),* );
                Some(r)
            }
        }
    }
}

macro_rules! try_opt {
    ($e:expr) => (match $e { Some(e) => e, None => return None })
}

// implement TryCollect<T> where T is a tuple with size 1, 2, .., 10
impl_try_collect_tuple!(A A A A A A A A A A);