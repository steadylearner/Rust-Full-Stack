// Should find "How to use bolt CLI with go mod"?
// https://gowebexamples.com/
// https://github.com/gorilla/mux

package main

import (
	// "encoding/json"

	"fmt"
	"log"
	"net/http"

	// "os"
	"time"

	"github.com/gorilla/mux"
	bolt "go.etcd.io/bbolt"
)

// Make a route for

// Create
// Read
// List
// Update
// Delete

// With User and return JSON or empty data while you refer to SQLite app.

func getPerson(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	id := vars["id"]

	db, err := bolt.Open("bolt.db", 0600, &bolt.Options{Timeout: 1 * time.Second})
	if err != nil {
		log.Fatal(err)
	}

	var person []byte
	if err := db.View(func(tx *bolt.Tx) error {
		person = tx.Bucket([]byte("people")).Get([]byte(id))
		return nil
	}); err != nil {
		log.Fatal(err)
	}

	if err := db.Close(); err != nil {
		log.Fatal(err)
	}

	fmt.Fprintf(w, "The %s person is %s.\n", id, person)
}

// $go run main.go
// $curl localhost:8000/people/1

func main() {
	router := mux.NewRouter()
	router.HandleFunc("/people/{id}", getPerson).Methods("GET")
	log.Fatal(http.ListenAndServe(":8000", router))
}
