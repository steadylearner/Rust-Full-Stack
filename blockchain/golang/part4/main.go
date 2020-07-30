// https://en.bitcoin.it/wiki/Transaction
// Refer to https://github.com/steadylearner/Rust-Full-Stack/tree/master/golang

// $go mod init https: //github.com/steadylearner/part4
// $go run *.go createblockchain -address Steadylearner

package main

func main() {
	cli := CLI{}
	cli.Run()
}
