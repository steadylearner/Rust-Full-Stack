package main

import (
    "flag"
    "fmt"
)

func main() {
    // Register Int flag. count is a pointer to it.
    count := flag.Int("count", 5, "count of iterations")
    // Parse the flags.
    flag.Parse()

    // Is seprating this meaningful?
    // Print the argument.
    fmt.Println("Argument", *count)

    // Get int from the Int pointer.
    value := *count
    fmt.Println("Value", value)
}
