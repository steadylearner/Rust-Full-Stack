package main

import "fmt"

func main() {
    // Step 1: create empty collection.
    values := [][]int{}

    // Step 2: these are the first two rows.
    // ... Append each row to the two-dimensional slice.
    row1 := []int{1, 2, 3}
    row2 := []int{4, 5, 6}
    values = append(values, row1)
    values = append(values, row2)

    // Step 3: display first row, and second row.
    fmt.Println("Row 1")
    fmt.Println(values[0])
    fmt.Println("Row 2")
    fmt.Println(values[1])

    // Step 4: access an element.
    fmt.Println("First element")
    fmt.Println(values[0][0])
}
