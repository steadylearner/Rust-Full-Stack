// https://www.programming-books.io/essential/go/wait-for-goroutines-to-finish-ea3629ac73bb494283d0c92b2a4f78d1

package main

import (
	"fmt"
	"sync"
)

// :show start
var wg sync.WaitGroup // 1

func routine(i int) {
	defer wg.Done() // 3
	fmt.Printf("routine %v finished\n", i)
}

func main() {
	wg.Add(10) // 2
	for i := 0; i < 10; i++ {
		go routine(i) // *
	}
	wg.Wait() // 4
	fmt.Println("main finished")
}
