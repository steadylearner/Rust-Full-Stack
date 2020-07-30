package main

import "fmt"

func main() {
	array := []int{10, 20, 30}
	for i := 0; i < len(array); i++ {
		fmt.Println(array[i])
	}
}
