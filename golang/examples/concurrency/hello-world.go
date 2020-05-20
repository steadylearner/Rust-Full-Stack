package main

import (
	"fmt"
	"time"
)

func main() {
	// create new channel of type string
	ch := make(chan string)

	// start new anonymous goroutine
	go func() {
		time.Sleep(time.Second)
		// send "Hello World" to channel
		ch <- "Hello World"
	}()

	// read from channel
	msg, ok := <-ch
	fmt.Printf("msg='%s', ok='%v'\n", msg, ok)
}

