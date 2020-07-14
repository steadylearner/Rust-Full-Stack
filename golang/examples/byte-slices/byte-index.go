package main

import (
	"bytes"
	"fmt"
)

func main() {
	values := []byte("a dog")

	// Search for this byte sequence.
	result := bytes.Index(values, []byte("dog"))
	fmt.Println(result)

	// This byte sequence is not found.
	result = bytes.Index(values, []byte("cat"))
	if result == -1 {
		fmt.Println("No cat in result.")
	}
}
