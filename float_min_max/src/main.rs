// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
// https://www.google.com/search?&q=how+to+find+min+and+max+f64+in+rust

use std::f64;

trait FloatIterExt {
    fn float_min(&mut self) -> f64;
    fn float_max(&mut self) -> f64;
}

// Floating point type doesn't implement Ord.
// So you write more code to make it work with NAN.
impl<T> FloatIterExt for T where T: Iterator<Item=f64> {
    // https://doc.rust-lang.org/std/primitive.f64.html#method.max
    // Recursively compare it.
    fn float_max(&mut self) -> f64 {
        self.fold(f64::NAN, f64::max)
    }
    
    // https://doc.rust-lang.org/std/primitive.f64.html#method.min
    // The same happens here
    fn float_min(&mut self) -> f64 {
        self.fold(f64::NAN, f64::min)
    }
}

fn main() {
    let vector = vec![0f64, 1f64, 2f64];
   
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.cloned
    // Should find why the original author used cloned here if necessary.
    let v_max = vector.iter().cloned().float_max();
    let v_min = vector.iter().cloned().float_min();
    
    println!("Max: {}", v_max);
    println!("Min: {}", v_min);
}
