# Find the latest posts from Rust subreddit.

[![Rust Telegram bot](https://github.com/steadylearner/Rust-Full-Stack/blob/master/bots/teloxide/rust_example.png)](https://github.com/steadylearner/Rust-Full-Stack/tree/master/bots/teloxide/src)

## I used teleoxide instead of teloxide mistakenly.

[Refer to this.](https://linuxize.com/post/how-to-use-sed-to-find-and-replace-string-in-files/)

```console
// 1. I used this to correct it.
$find . -type f -print0 | xargs -0 sed -i 's/teleoxide/teloxide/g'
// 2. Then, verified the result with these.
$grep -R "teleoxide" . // Should return empty result.
$grep -R "teloxide" 
```
