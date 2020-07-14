package main

import (
	"fmt"
	"sync"
	"time"
)

var (
	semaphoreSize = 4

	mu                 sync.Mutex
	totalTasks         int
	curConcurrentTasks int
	maxConcurrentTasks int
)

func timeConsumingTask() {
	mu.Lock()
	totalTasks++
	curConcurrentTasks++
	if curConcurrentTasks > maxConcurrentTasks {
		maxConcurrentTasks = curConcurrentTasks
	}
	mu.Unlock()

	// in real system this would be a CPU intensive operation
	time.Sleep(10 * time.Millisecond)

	mu.Lock()
	curConcurrentTasks--
	mu.Unlock()
}

func main() {
	sem := make(chan struct{}, semaphoreSize)
	var wg sync.WaitGroup
	for i := 0; i < 32; i++ {
		// acquire semaphore
		sem <- struct{}{}
		wg.Add(1)

		go func() {
			timeConsumingTask()
			// release semaphore
			<-sem
			wg.Done()
		}()
	}

	// wait for all task to finish
	wg.Wait()

	fmt.Printf("total tasks         : %d\n", totalTasks)
	fmt.Printf("max concurrent tasks: %d\n", maxConcurrentTasks)
}
