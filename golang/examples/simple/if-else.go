package main

import "fmt"

func main() {

	money := 0
	time := 0

	for {
		if time == 100000 {
			fmt.Println("Rest.")
			break
		}
		if money == 0 {
			fmt.Println("Work.")
			money++
			time++
		} else {
			fmt.Println("Spend")
			money--
			time++
		}
	}
}
