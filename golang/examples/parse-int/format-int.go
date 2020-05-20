package main

import (
   "fmt"
   "strconv"
)

func main() {
  value := 1055

  result := strconv.FormatInt(int64(value), 10)
  if result == "1055" {
    fmt.Println(true)
  }
}
