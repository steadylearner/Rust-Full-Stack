package main

import (
	"bytes"
	"encoding/binary"
	"log"
)

// IntToHex converts an int64 to a byte array
// https://www.google.com/search?client=firefox-b-d&q=what+is+endianness
func IntToHex(num int64) []byte {
	buff := new(bytes.Buffer)
	err := binary.Write(buff, binary.BigEndian, num)
	if err != nil {
		log.Panic(err)
	}

	return buff.Bytes()
}
