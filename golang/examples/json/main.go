package main

import (
	"encoding/json"
	"fmt"
	"log"
)

// `json: something` to use another format.
type Person struct {
	fullName string
	Name     string
	Age      int    `json:"age"`
	City     string `json:"city"`
}

func main() {
	// You can skip some fields.
	p := Person{
		Name: "John",
		Age:  37,
		City: "SF",
	}

	d, err := json.Marshal(&p)
	if err != nil {
		log.Fatalf("json.MarshalIndent failed with '%s'\n", err)
	}

	fmt.Printf("Person in compact JSON: %s\n", string(d))

	// https://golang.org/pkg/encoding/json/#MarshalIndent
	d, err = json.MarshalIndent(p, "", "  ") // prefix, indent
	if err != nil {
		log.Fatalf("json.MarshalIndent failed with '%s'\n", err)
	}
	fmt.Printf("Person in pretty-printed JSON:\n%s\n", string(d))
}
