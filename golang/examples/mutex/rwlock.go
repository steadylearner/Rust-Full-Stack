package main

import (
	"fmt"
	"sync"
	"time"
)

var cache map[int]int
// Exclusive
var mu sync.RWMutex

func expensiveOperation(n int) int {
	// in real code this operation would be very expensive
	return n * n
}

func getCached(n int) int {
        // Mutex guarantee memory safety.
	mu.RLock()
	v, isCached := cache[n]
	mu.RUnlock()
	if isCached {
		return v
	}

	v = expensiveOperation(n)

	mu.Lock()
	cache[n] = v
	mu.Unlock()
	return v
}

func accessCache() {
	total := 0
	for i := 0; i < 5; i++ {
		n := getCached(i)
		total += n
	}
	fmt.Printf("total: %d\n", total)
}

func main() {
	// :show start
	cache = make(map[int]int)
	go accessCache()
	accessCache()
	// :show end

	// for simplicity of the example
	// don't use time.Sleep() to coordinate goroutines
	// in production code
	time.Sleep(100 * time.Millisecond)
}

