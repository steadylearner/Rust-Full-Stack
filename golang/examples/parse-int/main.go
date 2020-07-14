package main

import (
  "fmt"
  "strconv"
)

func main() {
  value := "123"

  number, _ := strconv.ParseInt(value, 10, 0)
  if number == 123 {
     fmt.Println(true)
  }
}
