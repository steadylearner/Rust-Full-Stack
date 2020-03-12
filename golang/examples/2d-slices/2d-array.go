package main

import "fmt"

func main() {
    letters := [2][2]string{}

    letters[0][0] = "a"
    letters[0][1] = "b"
    letters[1][0] = "c"
    letters[1][1] = "d"

    fmt.Println(letters)
}
