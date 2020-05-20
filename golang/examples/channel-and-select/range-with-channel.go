package main

import (
	"fmt"
)

func foo(ch chan int) {
	ch <- 1
	ch <- 2
	close(ch)
}

func main() {
	ch := make(chan int)
	go foo(ch)
	for n := range ch {
		fmt.Println(n)
	}
	fmt.Println("channel is now closed")
}
