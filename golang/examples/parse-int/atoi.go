package main

import (
    "fmt"
    "strconv"
)

func main() {
    value := "456"
    // Use Atoi to parse string.
    number, _ := strconv.Atoi(value)

    fmt.Println(number + 1)
}
