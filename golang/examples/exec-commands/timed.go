package main

import (
	"fmt"
	"log"
	"os/exec"
	"sync/atomic"
	"time"
)

func main() {
	cmd := exec.Command("echo", "go")

	err := cmd.Start()
	if err != nil {
		log.Fatalf("cmd.Start() failed with '%s'\n", err)
	}

	var timedOut int32
	// timeout := 1 * time.Second // didTimeout: false, err: <nil>
	timeout := 1 * time.Millisecond // didTimeout: true, err: signal: killed
	stopTimer := time.AfterFunc(timeout, func() {
		cmd.Process.Kill()
		atomic.StoreInt32(&timedOut, 1)
	})

	err = cmd.Wait()
	stopTimer.Stop()
	// https://golang.org/pkg/sync/atomic/#LoadInt32
	// https://www.google.com/search?&q=what+does+atomically+load+the+data
	// Objects of atomic types are free from data races;
	didTimeout := atomic.LoadInt32(&timedOut) != 0
	fmt.Printf("didTimeout: %v, err: %v\n", didTimeout, err)
}


