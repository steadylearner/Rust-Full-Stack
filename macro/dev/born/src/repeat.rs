// https://github.com/rust-lang/rust/blob/master/src/libstd/macros.rs
// https://www.google.com/search?client=firefox-b-d&q=what+is+token+tree+in+rust
// https://doc.rust-lang.org/rust-by-example/trait/derive.html
// https://www.google.com/search?client=firefox-b-d&q=nested macro not allowed in rust
// https://www.google.com/search?client=firefox-b-d&q=struct inheritance vs composition rust

macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)* }
        __nested_macro!($);
    }
}

// Include public
// https://stackoverflow.com/questions/34373169/how-do-i-create-a-rust-macro-with-optional-parameters-using-repetitions
// https://users.rust-lang.org/t/best-way-to-make-a-macro-with-required-and-optional-arguments/27514
// (You can use optional.)
// https://doc.rust-lang.org/reference/macros-by-example.html#metavariables
macro_rules! make_base {
    (struct $basestruct:tt { $( $commonfield:ident: $commonty:ty ),* $(,)* }) => {
        nested_macro! {
            ($s:tt) => {
                macro_rules! $basestruct {
                    () => {
                        struct $basestruct {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (pub) => {
                        pub struct $basestruct {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)]) => {
                        #[derive($s($arg)+)]
                        struct $basestruct {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub) => {
                        #[derive($s($arg)+)]
                        pub struct $basestruct {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (struct $name:ident { $s( $field:ident: $ty:ty ),* $s(,)* }) => {
                        struct $name {
                            $( $commonfield: $commonty, )*
                            $s( $field: $ty ),*
                        }
                    };
                    (pub struct $name:ident { $s( $field:ident: $ty:ty ),* $s(,)* }) => {
                        pub struct $name {
                            $( $commonfield: $commonty, )*
                            $s( $field: $ty ),*
                        }
                    };
                    (#[derive($s($arg:tt)+)] struct $name:ident { $s( $field:ident: $ty:ty ),* $s(,)* }) => {
                        #[derive($s($arg)+)]
                        struct $name {
                            $( $commonfield: $commonty, )*
                            $s( $field: $ty ),*
                        }
                    };
                    (#[derive($s($arg:tt)+)] pub struct $name:ident { $s( $field:ident: $ty:ty ),* $s(,)* }) => {
                        #[derive($s($arg)+)]
                        pub struct $name {
                            $( $commonfield: $commonty, )*
                            $s( $field: $ty ),*
                        }
                    };
                    (reuse struct $name:ident ) => {
                        struct $name {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (reuse pub struct $name:ident ) => {
                        pub struct $name {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)] reuse struct $newname:ident ) => {
                        #[derive($s($arg)+)]
                        struct $newname {
                            $( $commonfield: $commonty, )*
                        }
                    };
                    (#[derive($s($arg:tt)+)] reuse pub struct $newname:ident ) => {
                        #[derive($s($arg)+)]
                        struct $newname {
                            $( $commonfield: $commonty, )*
                        }
                    };
                }
            }
        }
    };
}

make_base!(
    struct Example {
        id: u64,
        meta: f32,
    }
);

// Example!(); // Make Example struct
Example!(pub); // Make Example struct

// Example!(reuse struct ExampleCreateReply);

Example!(reuse struct Reuse);
// Example!(reuse pub struct Reuse);

Example!(
    #[derive(Debug, Clone, PartialEq)]
    reuse struct It
);

// Example!(
//     #[derive(Debug, Clone, PartialEq)]
//     reuse pub struct It
// );

make_base!(
    struct Steadylearner {
        javascript: bool,
        python: bool,
        rust: bool,
        go: bool,
    }
);

Steadylearner!(
    #[derive(Debug, Clone, PartialEq)]
);

// Steadylearner!(
//     #[derive(Debug, Clone, PartialEq)] 
//     pub 
// );

impl Steadylearner {
    fn have_npm_pacakge(&mut self, verify: bool) {
        self.javascript = verify
    }
    fn have_python_pacakge(&mut self, verify: bool) {
        self.rust = verify
    }
    fn have_cargo_pacakge(&mut self, verify: bool) {
        self.rust = verify
    }
    fn have_go_pacakge(&mut self, verify: bool) {
        self.go = verify
    }
}

Example!(
    struct Example0 {
        x: f32,
        y: f32,
    }
);

// Example!(
//     pub struct Example0 {
//         x: f32,
//         y: f32,
//     }
// );

Example!(
    #[derive(Debug, Clone, PartialEq)]
    struct DeriveExample0 {
        a: f32,
        b: f32,
    }
);

// Example!(
//     #[derive(Debug, Clone, PartialEq)]
//     pub struct DeriveExample0 {
//         a: f32,
//         b: f32,
//     }
// );

impl DeriveExample0 {
    fn sum_f32(&self) -> f32 {
        &self.a + &self.b
    }
}

fn sum_f32(a: &f32, b: &f32) -> f32 {
   let result = a + b;
   result
}

fn main() {
    let example = Example {
        id: 1,
        meta: 1.0
    };

    println!("{}", example.id);
    println!("{}", example.meta);

    let example0 = Example0 {
        id: 1,
        meta: 1.0,
        x: 1.0,
        y: 1.0,
    };

    println!("{}", example0.id);
    println!("{}", example0.meta);
    println!("{}", example0.x);
    println!("{}", example0.y);
    // println!("{:#?}", example0); fails

    let derive_example = DeriveExample0 {
        id: 1,
        meta: 1.0,
        a: 1.0,
        b: 1.0,
    };

    assert_eq!(derive_example, derive_example.clone());
    println!("{}", &derive_example.sum_f32());
    println!("{}", sum_f32(&derive_example.a, &derive_example.b));

    let mut steadylearner = Steadylearner {
        javascript: false,
        rust: false,
        python: false,
        go: false,
    };

    steadylearner.have_npm_pacakge(true);
    steadylearner.have_cargo_pacakge(true);

    assert_eq!(steadylearner.javascript, true);
    assert_eq!(steadylearner.rust, true);
    assert_eq!(steadylearner, steadylearner.clone());

    let reuse = Reuse {
        id: 1,
        meta: 1.0
    };

    println!("{}", reuse.id);
    println!("{}", reuse.meta);

    let it = It {
        id: 1,
        meta: 1.0
    };

    assert_eq!(it, it.clone());
}
