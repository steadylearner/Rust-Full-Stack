macro_rules! base {
    (struct $name:ident { $( $field:ident: $ty:ty ),* $(,)* }) => {
        #[derive(Debug)] // How to use it dynamically here?
        struct $name {
            id: u64,
            meta: f32,
            $( $field: $ty ),*
        }
    };
}

base!(struct Example1 {
    x: f32,
    y: f32,
}, ["Debug"]);

// Test impl here.

impl Example1 {
    fn sum_f32(&self) -> f32 {
        &self.x + &self.y
    }
}

base!(struct Example2 {
    a: f32,
    b: f32,
}, ["Debug"]);

// Test function here.
fn sum_f32(a: &f32, b: &f32) -> f32 {
   let result = a + b;
   result
}

fn main() {
    let example1 = Example1 {
        id: 1,
        meta: 1.0,
        x: 1.0,
        y: 1.0,
    };
   
    let example2 = Example2 {
        id: 1,
        meta: 1.0,
        a: 1.0,
        b: 1.0,
    };
    println!("{}", example1.id);
    println!("{:#?}", example1);
    println!("{}", example2.id);
    println!("{:#?}", example2);
  
    println!("{}", &example1.sum_f32()); 
    println!("{}", sum_f32(&example2.a, &example2.b));
}
