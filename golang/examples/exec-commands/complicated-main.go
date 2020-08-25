package main

import (
	"bytes"
	"fmt"
	"log"
	"os/exec"
)

func main() {
	var stdout, stderr bytes.Buffer
	cmd := exec.Command("echo", "go")
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr

	err := cmd.Start()
	if err != nil {
		log.Fatalf("cmd.Start() failed with '%s'\n", err)
	}

	err = cmd.Wait()
	if err != nil {
		log.Fatalf("cmd.Wait() failed with '%s'\n", err)
	}
	out := append(stdout.Bytes(), stderr.Bytes()...)
	fmt.Printf("%s", string(out))
}
