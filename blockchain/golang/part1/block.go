package main

import (
	"bytes"
	"crypto/sha256"
	"strconv"
	"time"
)

// Block keeps block headers
type Block struct {
	Timestamp     int64
	Data          []byte
	PrevBlockHash []byte
	Hash          []byte
}

// https://www.dotnetperls.com/bytes-go

// SetHash calculates and sets block hash
func (b *Block) SetHash() { // method
	timestamp := []byte(strconv.FormatInt(b.Timestamp, 10)) // byte slice
	headers := bytes.Join([][]byte{b.PrevBlockHash, b.Data, timestamp}, []byte{}) // Join byte slices with empty byte slice("")
	hash := sha256.Sum256(headers) // search Sum256

	b.Hash = hash[:] // copy the hash?
}

// NewBlock creates and returns Block
func NewBlock(data string, prevBlockHash []byte) *Block { // init
	block := &Block{time.Now().Unix(), []byte(data), prevBlockHash, []byte{}} // empty byte slice
	block.SetHash()
	return block
}

// NewGenesisBlock creates and returns genesis Block
func NewGenesisBlock() *Block { // init
	return NewBlock("Genesis Block", []byte{})
}
