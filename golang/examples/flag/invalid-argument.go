// $go run invalid-argument.go -size="show me the code"

// invalid value "show me the code" for flag -size: parse error
// Usage of /tmp/go-build486598432/b001/exe/invalid-argument:
//  -size int
//    	file size
// exit status 2


package main

import (
    "flag"
    "fmt"
)

func main() {
    // A string flag.
    size := flag.Int("size", 0, "file size")
    flag.Parse()

    // Print size.
    fmt.Println(*size)
}

