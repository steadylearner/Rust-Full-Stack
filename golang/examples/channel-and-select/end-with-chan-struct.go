// https://www.programming-books.io/essential/go/signaling-channel-with-chan-struct-f5daa999f5134b9ba9f2d69916df292a

// Is there a meaningful usecase of this?

package main

import "fmt"

func worker(ch chan int, chQuit chan struct{}) {
	for {
		select {
		case v := <-ch:
			fmt.Printf("Got value %d\n", v)
		case <-chQuit:
			fmt.Printf("Signalled on quit channel. End\n")
			chQuit <- struct{}{}
			return
		}
	}
}
func main() {
	ch, chQuit := make(chan int), make(chan struct{})
	go worker(ch, chQuit)
	ch <- 3
	chQuit <- struct{}{}

	// wait to be signalled back by the worker
	<-chQuit
}
