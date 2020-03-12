// Use unbuffered when the data is undefined.
// Use buffered when the data is defined.

package main

import (
	"fmt"
	"time"
)

func producer(ch chan int) {
	for i := 0; i < 5; i++ {
		if i%2 == 0 {
			time.Sleep(10 * time.Millisecond)
		} else {
			time.Sleep(1 * time.Millisecond)
		}
		ch <- i
	}
}

func consumer(ch chan int) {
	total := 0
	for i := 0; i < 5; i++ {
		if i%2 == 1 {
			time.Sleep(10 * time.Millisecond)
		} else {
			time.Sleep(1 * time.Millisecond)
		}
		total += <-ch
	}
}

func unbuffered() {
	timeStart := time.Now()
	ch := make(chan int)
	go producer(ch)
	consumer(ch)
	fmt.Printf("Unbuffered version took %s\n", time.Since(timeStart))
}

func buffered() {
	timeStart := time.Now()
	ch := make(chan int, 5)
	go producer(ch)
	consumer(ch)
	fmt.Printf("Buffered version took %s\n", time.Since(timeStart))
}

func main() {
	unbuffered()
	buffered() // faster
}
