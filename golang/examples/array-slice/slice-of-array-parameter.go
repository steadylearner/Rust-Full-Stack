package main

import "fmt"

func display(values []int) {
    // Loop over slice argument and display elements.
    for i:= 0; i < len(values); i++ {
        fmt.Println(values[i])
    }
}

func main() {
    // Create a four-element array.
    array := [...]int{-1, 0, 10, 100}
    // Pass a slice of the array to display.
    // ... This slice contains all elements.
    display(array[:])
}

