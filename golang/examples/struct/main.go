package main

import "fmt"

type Location struct {
    x     int
    y     int
    valid bool
}

func main() {
    // Create a new Location struct instance.
    loc := new(Location)

    // Set a field and then print its value.
    loc.x = 10
    fmt.Println(loc.x)
}
