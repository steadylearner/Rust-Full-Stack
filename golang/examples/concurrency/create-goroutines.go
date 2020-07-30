package main

import (
	"fmt"
	"time"
)

func mult(x, y int) {
	fmt.Printf("%d * %d = %d\n", x, y, x*y)
}

func main(){
	go mult(1, 2)
	go mult(3, 4)

	time.Sleep(200 * time.Millisecond)
}
