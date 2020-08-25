// $go run *.go
// Take a long time with the proofofwork

// Prev. hash:
// Data: Genesis Block
// Hash: 00000093253acb814afb942e652a84a8f245069a67b5eaa709df8ac612075038
// PoW: true

// Prev. hash: 00000093253acb814afb942e652a84a8f245069a67b5eaa709df8ac612075038
// Data: Send 1 BTC to www.steadylearner.com/blog
// Hash: 0000003eeb3743ee42020e4a15262fd110a72823d804ce8e49643b5fd9d1062b
// PoW: true

// Prev. hash: 0000003eeb3743ee42020e4a15262fd110a72823d804ce8e49643b5fd9d1062b
// Data: Send 2 more BTC to www.steadylearner.com/blog
// Hash: 000000e42afddf57a3daa11b43b2e0923f23e894f96d1f24bfd9b8d2d494c57a
// PoW: true

// https://keepingstock.net/explaining-blockchain-how-proof-of-work-enables-trustless-consensus-2abed27f0845

package main

import (
	"fmt"
	"strconv"
)

func main() {
	bc := NewBlockchain()

	bc.AddBlock("Send 1 BTC to www.steadylearner.com/blog")
	bc.AddBlock("Send 2 more BTC to www.steadylearner.com/blog")

	for _, block := range bc.blocks {
		fmt.Printf("Prev. hash: %x\n", block.PrevBlockHash)
		fmt.Printf("Data: %s\n", block.Data)
		fmt.Printf("Hash: %x\n", block.Hash)
		pow := NewProofOfWork(block)
		fmt.Printf("PoW: %s\n", strconv.FormatBool(pow.Validate())) // bool to string
		fmt.Println()
	}
}

// https://www.dotnetperls.com/array-go
// https://www.dotnetperls.com/2d-go
