// https://github.com/etcd-io/bbolt#reading-the-source
// https://www.google.com/search?q=golang+closure

// $go mod init https: //github.com/steadylearner/part3
// $go run *.go printchain

package main

func main() {
	bc := NewBlockchain()
	defer bc.db.Close()

	cli := CLI{bc}
	cli.Run()
}
