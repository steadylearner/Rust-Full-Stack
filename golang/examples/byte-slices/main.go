// https://www.dotnetperls.com/bytes-go

package main

import "fmt"

func main() {
    values := []byte("abc")
    fmt.Println(values)

    // Append a byte.
    values = append(values, byte('d'))

    // Print string representation of bytes.
    fmt.Println(string(values))

    // Length of byte slice.
    fmt.Println(len(values))

    // First byte.
    fmt.Println(values[0])
}
