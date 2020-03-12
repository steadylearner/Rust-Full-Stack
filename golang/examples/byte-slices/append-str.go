package main

import "fmt"

func main() {
    value := []byte("abc")
    // Append a string to a byte slice with special syntax.
    value = append(value, "def"...)
    fmt.Println(value)
}

