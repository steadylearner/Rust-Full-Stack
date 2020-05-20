package main

import (
  "fmt"
  "runtime"
)

func main() {
   fmt.Printf("%v\n", runtime.NumCPU())
}
