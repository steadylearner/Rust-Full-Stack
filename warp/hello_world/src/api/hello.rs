// https://stackoverflow.com/questions/29068716/how-do-you-use-a-macro-from-inside-its-own-crate
// Macros defined this way become gloally available with #[macro_export]
// When the module containing it is included at main.rs or lib.rs

// It seems it is more important for macro "Where it is used" than "Where it is defined"
#[macro_export]
macro_rules! hello {
    () => {
        hello_route::hello()
        .and_then(hello_handler::hello)
    }
}