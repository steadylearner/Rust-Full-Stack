// // https://github.com/rust-lang/rust/blob/master/src/libstd/macros.rs
// // https://www.google.com/search?client=firefox-b-d&q=what+is+token+tree+in+rust
// // https://doc.rust-lang.org/rust-by-example/trait/derive.html
// // https://www.google.com/search?client=firefox-b-d&q=nested macro not allowed in rust
// // https://www.google.com/search?client=firefox-b-d&q=struct inheritance vs composition rust

// macro_rules! nested_macro {
//     ($($body:tt)*) => {
//         macro_rules! __nested_macro { $($body)* }
//         __nested_macro!($);
//     }
// }

// macro_rules! private_enum {
//     (enum $basesenum:ident { $( $commonfield:tt ),+ $(,)* }) => {
//         nested_macro! {
//             ($s:tt) => {
//                 macro_rules! $basesenum {
//                     () => {
//                         enum $basesenum {
//                             $( $commonfield, )*
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)]) => {
//                         #[derive($s($arg)+)]
//                         enum $basesenum {
//                             $( $commonfield, )*
//                         }
//                     };

//                     (enum $name:ident { $s( $s($field:tt)* ),* $s(,)* }) => {
//                         enum $name {
//                             $( $commonfield, )*
//                             $s( $field ),*
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)] pub enum $name:ident { $s( $field:tt ),+ $s(,)* }) => {
//                     // (#[derive($s($arg:tt)+)] enum $name:ident { $s( $s($field:tt)* ),* $s(,)* }) => {
//                         #[derive($s($arg)+)]
//                         enum $name {
//                             $( $commonfield, )*
//                             $s( $field ),*
//                         }
//                     };

//                     (enum $name:ident) => {
//                         enum $name {
//                             $( $commonfield, )*
//                         }
//                     };
//                     (#[derive($s($arg:tt)+)] enum $name:ident) => {
//                         #[derive($s($arg)+)]
//                         enum $name {
//                             $( $commonfield, )*
//                         }
//                     };
//                 }
//             }
//         }
//     };
// }

// // Code from Rust
// // https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html

// private_enum!(
//     enum WebEventBase {
//         PageLoad,
//         PageUnload,
//     }
// );

// WebEventBase!(
//     #[derive(Debug, Clone, PartialEq)]
//     enum WebEvent {
//         KeyPress(char),
//         Paste(String),
//         // or c-like structures.
//         Click { x: i64, y: i64 },
//     }
// );

// // A function which takes a `WebEvent` enum as an argument and
// // returns nothing.
// fn inspect(event: WebEvent) {
//     match event {
//         WebEvent::PageLoad => println!("page loaded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         // Destructure `c` from inside the `enum`.
//         WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
//         WebEvent::Paste(s) => println!("pasted \"{}\".", s),
//         // Destructure `Click` into `x` and `y`.
//         WebEvent::Click { x, y } => {
//             println!("clicked at x={}, y={}.", x, y);
//         },
//     }
// }

// fn main() {
//     let pressed = WebEvent::KeyPress('x');
//     // `to_owned()` creates an owned `String` from a string slice.
//     let pasted  = WebEvent::Paste("my text".to_owned());
//     let click   = WebEvent::Click { x: 20, y: 80 };
//     let load    = WebEvent::PageLoad;
//     let unload  = WebEvent::PageUnload;

//     inspect(pressed);
//     inspect(pasted);
//     inspect(click);
//     inspect(load);
//     inspect(unload);
// }

// // https://danielkeep.github.io/tlborm/book/blk-enum-parsing.html
// // macro_rules! public_enum {
// //     (pub enum $basesenum:ident { $( $($commonfield:tt)* ),* $(,)* }) => {
// //         nested_macro! {
// //             ($s:tt) => {
// //                 macro_rules! $basesenum {
// //                     () => {
// //                         pub enum $basesenum {
// //                             $( $commonfield, )*
// //                         }
// //                     };
// //                     (#[derive($s($arg:tt)+)]) => {
// //                         #[derive($s($arg)+)]
// //                         pub enum $basesenum {
// //                             $( $commonfield, )*
// //                         }
// //                     };

// //                     (pub enum $name:ident { $s( $($field:tt)* ),* $(,)* }) => {
// //                         pub enum $name {
// //                             $( $commonfield, )*
// //                             $s( $field ),*
// //                         }
// //                     };
// //                     (#[derive($s($arg:tt)+)] pub enum $name:ident { $s( $($field:tt)* ),* $(,)* }) => {
// //                         #[derive($s($arg)+)]
// //                         pub enum $name {
// //                             $( $commonfield, )*
// //                             $s( $field ),*
// //                         }
// //                     };

// //                     (pub enum $name:ident) => {
// //                         pub enum $name {
// //                             $( $commonfield, )*
// //                         }
// //                     };
// //                     (#[derive($s($arg:tt)+)] pub enum $name:ident) => {
// //                         #[derive($s($arg)+)]
// //                         pub enum $name {
// //                             $( $commonfield, )*
// //                         }
// //                     };
// //                 }
// //             }
// //         }
// //     };
// // }

