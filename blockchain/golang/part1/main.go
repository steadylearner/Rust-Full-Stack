// $go fmt *.go
// $go run *.go
// $go run main.go block.go blockchain.go

package main

import (
	"fmt"
)

func main() {
	bc := NewBlockchain()

	bc.AddBlock("Send 1 BTC to www.steadylearner.com/blog")
	bc.AddBlock("Send 2 more BTC to www.steadylearner.com/blog")

	// https://golang.org/pkg/fmt/
	for _, block := range bc.blocks {
		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash) // hexademical
		fmt.Printf("Data: %s\n", block.Data) // string
		fmt.Printf("Hash: %x\n", block.Hash)
		fmt.Println()
	}
}

// Prev. hash:
// Data: Genesis Block
// Hash: 7641cc59327f7b08de2b878af41d2fee2329c4a076d68311a53278cd3149c5fd

// Prev. hash: 7641cc59327f7b08de2b878af41d2fee2329c4a076d68311a53278cd3149c5fd
// Data: Send 1 BTC to www.steadylearner.com/blog
// Hash: 91d6ed6fa5864b3bbaa78b77e415f5501cb05d1555e0487f018299b69ff3c2c0

// Prev. hash: 91d6ed6fa5864b3bbaa78b77e415f5501cb05d1555e0487f018299b69ff3c2c0
// Data: Send 2 more BTC to www.steadylearner.com/blog
// Hash: 7e1d6d0a4a7942ad5e1907c9d66fc09687ffbcad1d283476dbb7571663be87d7
