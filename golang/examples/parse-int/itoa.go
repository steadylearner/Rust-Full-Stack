package main

import (
    "fmt"
    "strconv"
)

func main() {
    value := 700

    // Use Itoa on an int.
    result := strconv.Itoa(value)
    fmt.Println(result)

    // The string has 3 characters.
    fmt.Println(len(result))
}
