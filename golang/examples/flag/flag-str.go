package main

import (
     "flag"
     "fmt"
     "strings"
)

func main() {
  greeting := flag.String("greeting", "Hello", "startup message")
  flag.Parse()

  value := *greeting
  fmt.Println("Program says:", strings.ToUpper(value))
}
