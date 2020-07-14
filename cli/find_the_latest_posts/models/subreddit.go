package models

// Refer to this.
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/bots/teloxide/src/community-bots/models/subreddit.rs

type Post struct {
	Title string
	URL   string
}

type Child struct {
	Data Post
}

type Children struct {
	Children []Child
}

type Response struct {
	Data Children
}

// HTTP Error Responses
type NotFound struct {
	Message string
        Error int
}


