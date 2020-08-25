package main

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gorilla/mux"
	"github.com/stretchr/testify/assert"
)

func TestArticlesHandler(t *testing.T) {

	req, err := http.NewRequest("GET", "/posts", nil)

	checkError(err, t)

	rr := httptest.NewRecorder()

	//Make the handler function satisfy http.Handler
	//https://lanreadelowo.com/blog/2017/04/03/http-in-go/
	http.HandlerFunc(articlesHandler).
		ServeHTTP(rr, req)

	//Confirm the response has the right status code
	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Status code differs. Expected %d .\n Got %d instead", http.StatusOK, status)
	}

	//Confirm the returned json is what we expected
	//Manually build up the expected json string
	expected := string(`[{"id":1,"title":"New blog resolution","content":"I have decided to give my blog a new life and would hence forth try to write as often"},{"id":2,"title":"Go is cool","content":"Yeah i have been told that multiple times"},{"id":3,"title":"Interminttent fasting","content":"You should try this out, it helps clear the brain and tons of health benefits"},{"id":4,"title":"Yet another blog post","content":"I made a resolution earlier to keep on writing. Here is an affirmation of that"},{"id":5,"title":"Backpacking","content":"Yup, i did just that"}]`)

	//The assert package checks if both JSON string are equal and for a plus, it actually confirms if our manually built JSON string is valid
	assert.JSONEq(t, expected, rr.Body.String(), "Response body differs")
}

func TestArticleHandlerWithValidPost(t *testing.T) {
	req, err := http.NewRequest("GET", "/posts/2", nil)

	checkError(err, t)

	rr := httptest.NewRecorder()

	r := mux.NewRouter()

	r.HandleFunc("/posts/{id:[0-9]+}", articleHandler).Methods("GET")

	r.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Status code differs. Expected %d.\n Got %d", http.StatusOK, status)
	}

	expected := string(`{"id":2,"title":"Go is cool","content":"Yeah i have been told that multiple times"}`)

	assert.JSONEq(t, expected, rr.Body.String(), "Response body differs")

}

func TestArticleHandlerWithAnInvalidPost(t *testing.T) {

	req, err := http.NewRequest("POST", "/posts/42", nil)

	checkError(err, t)

	rr := httptest.NewRecorder()

	r := mux.NewRouter()

	r.HandleFunc("/posts/{id:[0-9]+}", articleHandler).Methods("GET")

	r.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusNotFound {
		t.Errorf("Status code differs. Expected %d \n. Got %d", http.StatusNotFound, status)
	}

	expected := "404 page not found\n"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")
}

func TestAnEmptyTitleCannotBeUsedToCreateANewPost(t *testing.T) {

	data := []byte(`{"title" : "", "content" : "A new blog post"}`)

	req, err := http.NewRequest("POST", "/posts", bytes.NewBuffer(data))

	checkError(err, t)

	rr := httptest.NewRecorder()

	http.HandlerFunc(createArticle).ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("Status code is invalid. Expected %d. Got %d instead", http.StatusBadRequest, status)
	}

	expected := "Invalid data... The title and/or content for a blog posts cannot be empty"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")
}

func TestAnEmptyContentCannotBeUsedToCreateANewPost(t *testing.T) {

	data := []byte(`{"title" : "Some blog title", "content" : ""}`)

	req, err := http.NewRequest("POST", "/posts", bytes.NewBuffer(data))

	checkError(err, t)

	rr := httptest.NewRecorder()

	http.HandlerFunc(createArticle).ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("Status code is invalid. Expected %d. Got %d instead", http.StatusBadRequest, status)
	}

	expected := "Invalid data... The title and/or content for a blog posts cannot be empty"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")
}

func TestCanCreateANewBlogPost(t *testing.T) {

	data := []byte(`{"title" : "Title of a new blog post", "content" : "A new blog post"}`)

	req, err := http.NewRequest("POST", "/posts", bytes.NewBuffer(data))

	checkError(err, t)

	rr := httptest.NewRecorder()

	http.HandlerFunc(createArticle).ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Status code is invalid. Expected %d. Got %d instead", http.StatusOK, status)
	}

	expected := "The blog post have been created"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")

	if len(posts) != 6 { //we appended one to the array
		t.Errorf("Post was not created")
	}
}

func TestCannotDeleteNonExistentBlogPost(t *testing.T) {
	req, err := http.NewRequest("DELETE", "/posts", bytes.NewBuffer([]byte(`{"id" : 60}`)))

	checkError(err, t)

	rr := httptest.NewRecorder()

	http.HandlerFunc(deleteArticleHandler).ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusNotFound {
		t.Errorf("Status code differs. Expected %d. Got %d", http.StatusNotFound, status)
	}

	expected := "Not Found"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")
}

func TestCanDeleteAPost(t *testing.T) {
	req, err := http.NewRequest("DELETE", "/posts", bytes.NewBuffer([]byte(`{"id" : 5}`)))

	checkError(err, t)

	rr := httptest.NewRecorder()

	http.HandlerFunc(deleteArticleHandler).ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Status code differs. Expected %d. Got %d", http.StatusOK, status)
	}

	expected := "The blog post was deleted successfully"

	assert.Equal(t, expected, rr.Body.String(), "Response body differs")

	if len(posts) != 4 {
		t.Errorf("An error occurred while post was being deleted, Post count is %d", len(posts))
	}

}

func checkError(err error, t *testing.T) {
	if err != nil {
		t.Errorf("An error occurred. %v", err)
	}
}
