// $go run raw-args steady learner

// Name of executable: '/tmp/go-build589628732/b001/exe/raw-args'
// Arg 0, value: 'steady'
// Arg 1, value: 'learner'

package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Printf("Name of executable: '%s'\n", os.Args[0])
	args := os.Args[1:]
	for i, arg := range args {
		fmt.Printf("Arg %d, value: '%s'\n", i, arg)
	}
}
