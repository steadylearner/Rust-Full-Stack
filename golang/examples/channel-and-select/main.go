package main

import (
	"fmt"
	"math/rand"
)

func genInts(chInts chan int) {
	chInts <- rand.Intn(1000)
}

func main() {
	chInts := make(chan int)
	for i := 0; i < 2; i++ {
		go genInts(chInts)
	}
	n := <-chInts
	fmt.Printf("n: %d\n", n)

	select {
	case n := <-chInts:
		fmt.Printf("n: %d\n", n)
	}
}
