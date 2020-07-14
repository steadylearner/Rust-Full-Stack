package main

import "fmt"

func main() {
    // Create an empty byte slice of length 4.
    values := make([]byte, 4)
    // Copy string into bytes.
    animal := "cat"
    copied := copy(values, animal)
    fmt.Println(copied)
    fmt.Println(values)
}
