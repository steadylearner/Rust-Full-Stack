// go run main.go -subreddit="golang" -limit="10"
// "https://www.reddit.com/r/golang/new/.json?limit=10"

package main

import (
	"bufio"
	"errors"
	"os"
	"strings"

	"fmt"
	"log"

	"github.com/go-resty/resty/v2"
	"github.com/logrusorgru/aurora"

	// https://golang.org/doc/code.html
	// $go build first in models/
	// Refer to go.mod file also.
	Models "steadylearner.com/models"
)

var (
	au aurora.Aurora
)

func init() {
	au = aurora.NewAurora(true)
}

func handleReadToString(err error) {
	if err != nil {
		log.Fatalf("reader.ReadString failed with '%s'\n", err)
	}
}

func removeNewLine(input string) string {
	return strings.TrimSuffix(input, "\n")
}

func main() {
	for {
		subreddit_reader := bufio.NewReader(os.Stdin)
		fmt.Println("Which subreddit you want to scrape?(golang)")
		subreddit, err := subreddit_reader.ReadString('\n')
		handleReadToString(err)
		if subreddit == "\n" {
			subreddit = "golang"
		}

		limit_reader := bufio.NewReader(os.Stdin)
		fmt.Println("How many you want?(50)") // [1, 100]
		limit, err := limit_reader.ReadString('\n')
		handleReadToString(err)
		if limit == "\n" {
			limit = "50"
		}

		target := fmt.Sprintf(
			"https://www.reddit.com/r/%s/new/.json?limit=%s",
			removeNewLine(subreddit),
			removeNewLine(limit),
		)

		// https://godoc.org/github.com/go-resty/resty#Request.SetResult
		// https://github.com/go-resty/resty/blob/master/response.go#L59
		resp, err := resty.New().R().
			SetResult(&Models.Response{}).
			Get(target)
		if err != nil {
			log.Fatal(err)
			return
		}

		// Type conversion to Models.Response so verify ok value here?
		if response, ok := resp.Result().(*Models.Response); !ok {
			log.Fatal(errors.New("invalid response format"))
		} else {
			for index, post := range response.Data.Children {
				num := index + 1
				stdoutLink := fmt.Sprintf("%d. %s (%s)", num, post.Data.Title, au.Blue(post.Data.URL))
				fmt.Println(stdoutLink)
			}
		}

		end_reader := bufio.NewReader(os.Stdin)
		fmt.Println("\nEnd?([n]/y])")
		end, err := end_reader.ReadString('\n')
		handleReadToString(err)
		if strings.HasPrefix(end, "y") {
			break
		}
	}
}
