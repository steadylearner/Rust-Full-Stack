// It has some features that doesn't work with stable. 
// So use this command first.
// $rustup default nightly

// Refer to this and use Rust instead of JavaScript.
// https://www.steadylearner.com/blog/read/How-to-use-Python-in-JavaScript

#![feature(proc_macro_hygiene)]
use inline_python::python;

fn main() {
    let author = "Steadylearner (https://www.steadylearner.com)";
    let n = 5;

    python! {
        import subprocess as cmd

        for i in range('n):
            print("Count: " + str(i + 1) + " with " + 'author)
        cp = cmd.run("echo 'End with built in Python module (import subprocess as cmd)'.", check=True, shell=True)

        print("\nTest with $pip -m venv rust and custom packages.")
    }
}
