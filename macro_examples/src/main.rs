// 1. https://doc.rust-lang.org/rust-by-example/macros.html, https://doc.rust-lang.org/stable/book/ch19-06-macros.html, https://danielkeep.github.io/practical-intro-to-macros.html, https://danielkeep.github.io/tlborm/book/index.html

// 2. Read some blog posts from https://www.google.com/search?&q=rust+macro+example
// 3. Refer to https://github.com/dtolnay/quote, https://github.com/dtolnay/syn and https://doc.rust-lang.org/proc_macro/ for procedual macros.
// 4. Clone and read the code of https://github.com/mjkillough/factori 

use std::collections::HashMap;

macro_rules! map {
    // => or, :
    ($( $key:expr => $value:expr ),*) => {{
        let mut hm = HashMap::new();
        $( hm.insert($key, $value); )*
        hm
    }}
}

fn main() {
    let steadylearner = map!(
       "name" => "Steadylearner",
       "language" => "Rust, Python, JavaScript, (Golang)",
       "website" => "https://www.steadylearner.com",
       "blog" => "https://www.steadylearner.com/blog/search/Rust",
       "portfolio" => "https://github.com/steadylearner/Rust-Full-Stack",
       "wanting_to_work_with_rust" => "true",
       "but_there_is_no_rust_jobs" => "true at least here",
       "will_someone_read_this_and_give_me_an_opportunity?" => "maybe if he or she thought my codes are useful."
    );
    println!("{:#?}", &steadylearner);
}
