// $go run main.go -echo echo-arg additional arg

package main

import (
	"flag"
	"fmt"
	"os"
)

// f is flag
var (
	fHelp bool
	fEcho string
)

func parseCmdLineFlags() {
	flag.BoolVar(&fHelp, "help", false, "if true, show help")
	flag.StringVar(&fEcho, "echo", "", "")
	flag.Parse()
}

func main() {
	parseCmdLineFlags()
	if fHelp {
		flag.Usage()
		os.Exit(0)
	}
	fmt.Printf("flag -echo: '%s'\n", fEcho)

	remainingArgs := flag.Args()
	for _, arg := range remainingArgs {
		fmt.Printf("Remainig arg: '%s'\n", arg)
	}
}
