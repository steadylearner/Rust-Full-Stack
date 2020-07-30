package main

import "fmt"

func display(values [3]int) {
    fmt.Println(values[0])
    fmt.Println(values[1])
    fmt.Println(values[2])
}

func main() {
    v := [...]int{5, 10, 15}
    // Pass the entire array to a method.
    // ... This copies the array.
    display(v)
}
