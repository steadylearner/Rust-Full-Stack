package main

import (
	"fmt"
	"sync"
)

// https://doc.rust-lang.org/std/sync/atomic/
// https://golang.org/pkg/sync/#WaitGroup
var wg sync.WaitGroup

func sqrtWorker(chIn chan int, chOut chan int) {
	fmt.Printf("sqrtWorker started\n")
	for i := range chIn {
		sqrt := i * i
		chOut <- sqrt
	}
	fmt.Printf("sqrtWorker finished\n")
	wg.Done()
}

func main() {
	chIn := make(chan int)
	chOut := make(chan int)
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go sqrtWorker(chIn, chOut)
	}

	// anoynmous goroutine
	go func() {
		chIn <- 2
		chIn <- 4
		close(chIn)
	}()

	// Treat separately.
	go func() {
		wg.Wait()
		close(chOut)
	}()

	for sqrt := range chOut {
		fmt.Printf("Got sqrt: %d\n", sqrt)
	}
}

// sqrtWorker started
// Got sqrt: 4
// sqrtWorker started
// sqrtWorker finished
// Got sqrt: 16
// sqrtWorker finished

