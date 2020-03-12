// This is where I test a language feature when I am in Rust Full Stack folder.

struct HasDrop;

impl HasDrop {
    fn print_something(&self) {
        println!("something");
    }
}

impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let x = HasDrop;
    x.print_something();
}
