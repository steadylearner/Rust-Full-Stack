package main

import (
	"fmt"
	"time"
)

func main() {
	chResult := make(chan int, 1)

	go func() {
		time.Sleep(1 * time.Second)
		chResult <- 5
		fmt.Printf("Worker ended")
	}()

	select {
	case res := <-chResult:
		fmt.Printf("Got %d from worker\n", res)
	case <-time.After(100 * time.Millisecond):
		fmt.Printf("Timed out before worker ended\n")
	}
}

