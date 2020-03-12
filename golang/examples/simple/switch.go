package main

import (
	"fmt"
	"time"
)

func main() {

	today := time.Now().Weekday()

	switch today {
	case time.Saturday, time.Sunday:
		fmt.Println("Read the documenation.")
	default:
		fmt.Println("Write code.")
	}
}
