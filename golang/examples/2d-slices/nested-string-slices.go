package main

import "fmt"

func main() {
    // Create an empty slice of slices.
    animals := [][]string{}

    // Create two string slices.
    row1 := []string{"www", "steady", "learner", "com", "blog"}
    row2 := []string{"www.steadylearner.com/blog"}

    // Append string slices to outer slice.
    animals = append(animals, row1)
    animals = append(animals, row2)

    // Use its meembers with for.
    for i, v := range animals {
        fmt.Printf("Row: %v\n", i)
        fmt.Println(v)
    }
}

