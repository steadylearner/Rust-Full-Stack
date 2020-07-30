macro_rules! nested_macro {
    ($($body:tt)*) => {
        macro_rules! __nested_macro { $($body)+ }
        __nested_macro!($);
    }
}