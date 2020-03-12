fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n // Use more if else or match here if you want.
    } else {
        // What matters is here for this simple example.        

        let mut counter = 1;
        let mut start = 0;
        let mut end = 1;

        loop {
            counter += 1;

            if counter == n {
                let last_fibonacci = start + end;
                break last_fibonacci;
            } else {
                let interim_fibonacci = start + end;
                start = end; // Previous end becomes the start value.
                end = interim_fibonacci; // end includes all the previous sum of the start and end.
            }
        }
    }
}

fn main() {
    let test_fibonacci = fibonacci(8);
    println!("{}", test_fibonacci);
}
