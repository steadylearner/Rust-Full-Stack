package main

import (
    "fmt"
    "net/http"
    "io/ioutil"
    // "os"
    "log"
)

func main() {
    target := "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md"

    response, err := http.Get(target)
    if err != nil {
        log.Fatal(err)
    } else {
        defer response.Body.Close()
        body, err := ioutil.ReadAll(response.Body)
        if err != nil {
            log.Fatal(err)
        }
        fmt.Printf("%s\n", string(body))
    }
}
