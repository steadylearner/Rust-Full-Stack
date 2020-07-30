// Golang version of
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/commit.py
// It is much simpler with Python.

// https://golang.org/pkg/strings/#HasPrefix

// Find idiomatic version of this. Then, make reset-commit.go
// What should I search?

// You should remove commit binary file if you made it before with this.
// $go build commit.go

// I will use $./commit normally. But, if you have a fast machine. Just use $go run .

// $go help packages

package main

import (
	"bufio"
	"fmt"
	"log"
	// Repeating os here. Is there way to import them simialr to os::{Stdin, exec} in Rust?
	// os and os/exec are separate packages. You can still use . "os" to use Stdin instead of os.Stdin
	"os"
	"os/exec"
	"strings"
)

func handleCombinedOutput(out []byte, err error) {
	if err != nil {
		log.Fatalf("cmd.CombinedOutput() failed with '%s'\n", err)
	}
	fmt.Printf("%s", string(out))
}

func handleReadToString(err error) {
	if err != nil {
		log.Fatalf("reader.ReadString failed with '%s'\n", err)
	}
}

func execCommand(name string, args ...string) {
	// How to spread arg here?
	// cannot use args (type []string) as type string in argument to exec.Command
	// https://stackoverflow.com/questions/32721066/pass-string-to-a-function-that-expects-a-variadic-parameter
	cmd := exec.Command(name, args...)
	out, err := cmd.CombinedOutput()
	handleCombinedOutput(out, err)
}

// cmd := exec.Command("git", "add", ".")
// Can I extract it to function?
// Is there a better way?

// Make this to function
// out, err := cmd.CombinedOutput()
// if err != nil {
// 	log.Fatalf("cmd.CombinedOutput() failed with '%s'\n", err)
// }
// fmt.Printf("%s", string(out))

// out, err := cmd.CombinedOutput()
// handleCombinedOutput(out, err)

func main() {
	execCommand("git", "add", ".")

	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Do you want to use the default message for this commit?([y]/n)\n")
	response, err := reader.ReadString('\n')
	// When the user want to use "Enter" here.
	if response == "\n" {
		response = "y"
	}
	handleReadToString(err)

	// var message = "Edit README.md"

	// if strings.HasPrefix(response, "n") {
	// 	reader := bufio.NewReader(os.Stdin)
	// 	fmt.Print("What do you want then?\n")
	// 	response, err := reader.ReadString('\n')
	// 	handleReadToString(err)
	// 	message = response
	// }

	var defaultMessage = "Edit README.md"

	if strings.HasPrefix(response, "n") {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("What do you want then?\n")
		response, err := reader.ReadString('\n')
		handleReadToString(err)

		// fmt.Print(response) "\n" when the user accidentally use "Enter" here.
		// Where to hanlde this "Aborting commit due to empty commit message."
		if response == "\n" {
			fmt.Println("The input is empty and the default 'Edit README.md' will be used here.\n")
			execCommand("git", "commit", "-m", defaultMessage)
		} else {
			execCommand("git", "commit", "-m", response)
		}
	} else {
		execCommand("git", "commit", "-m", defaultMessage)
	}

	execCommand("git", "push", "-u", "origin", "master", "-f")
}
