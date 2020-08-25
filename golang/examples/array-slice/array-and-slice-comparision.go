// https://www.dotnetperls.com/array-go

package main

import (
    "fmt"
    "time"
)

func main() {

    // Create array and slice.
    array := [...]int{10, 20, 30}
    slice := []int{10, 20, 30}
    sum := 0

    t0 := time.Now()

    // Version 1: assign into and read array elements.
    for i := 0; i < 1000000000; i++ {
        sum = 0
        for x := 0; x < len(array); x++ {
            array[x] = 5
            sum += array[x]
        }
        if sum == 0 {
            break
        }
    }

    t1 := time.Now()

    // Version 2: assign into and read slice elements.
    for i := 0; i < 1000000000; i++ {
        sum = 0
        for x := 0; x < len(slice); x++ {
            slice[x] = 5
            sum += slice[x]
        }
        if sum == 0 {
            break
        }
    }

    t2 := time.Now()
    // Results.
    fmt.Println(t1.Sub(t0))
    fmt.Println(t2.Sub(t1))
}

