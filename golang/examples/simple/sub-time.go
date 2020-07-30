package main

import (
    "fmt"
    "time"
)

func main() {
    t0 := time.Now()

    fmt.Println("Show me the code.")
    time.Sleep(1 * time.Second)

    t1 := time.Now()

    fmt.Println(t1.Sub(t0))
}

