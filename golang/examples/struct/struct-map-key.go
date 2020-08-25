package main

import "fmt"

type Measure struct {
    size int
    unit string
}

func main() {
    // A map with struct keys.
    storage := map[Measure]bool{}

    // Add 3 structs as keys in the map.
    m := new(Measure)
    m.size = 10
    m.unit = "centimeters"
    storage[*m] = true

    m = new(Measure)
    m.size = 20
    m.unit = "feet"
    storage[*m] = true

    m = new(Measure)
    m.size = 10
    m.unit = "decibels"
    storage[*m] = true

    // There are 3 keys in the map.
    fmt.Println("Map len", len(storage))

    // Create structs to look up values in the map.
    key := new(Measure)
    key.size = 10
    key.unit = "centimeters"
    v := storage[*key]
    fmt.Println("Result", key, v)

    key = new(Measure)
    key.size = 100
    key.unit = "decibels"
    v = storage[*key]
    fmt.Println("Result", key, v)
}
