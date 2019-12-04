package main

import (
	"fmt"
	"log"
	"net/http"

        . "github.com/logrusorgru/aurora"
)

// https://golang.org/pkg/net/http/#ServeFile
func singlePageApp(w http.ResponseWriter, r *http.Request) {
	http.ServeFile(w, r, "public/index.html")
}

func main() {
        // http.HandleFunc("/", singlePageApp) // Multiple registration warning.
	http.HandleFunc("/user", singlePageApp)

        public := http.FileServer(http.Dir("public"))
	http.Handle("/", public)
        target := Blue("http://0.0.0.0:8000")

	fmt.Printf("Server ready at %s\n", target)
	log.Println("Listening...")
	http.ListenAndServe(":8000", nil)
}
