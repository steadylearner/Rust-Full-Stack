# Texts

```console
https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest

https://github.com/steadylearner/Rust-Full-Stack/blob/master/bots/telegram/src/get/main.rs
https://gist.github.com/patshaughnessy/27b1611e2c912346b929df97998d488d
https://www.google.com/search?&q=concurrent+requests+rust+reqwest

https://docs.rs/futures/0.2.0/futures/future/fn.join_all.html
https://docs.rs/futures-util-preview/0.3.0-alpha.18/futures_util/future/fn.join_all.html
https://docs.rs/futures-util-preview/0.3.0-alpha.18/futures_util/stream/futures_unordered/struct.FuturesUnordered.html
```

```console
you use an http client, for example the one from the crate Reqwest, and combine the replies in a single future, for example with the function join_all from crate Futures_util

You can use the example from hyper https://hyper.rs/guides/client/basic/ . The only changes you should need to do is create multiple requests instead of one, then join all the requests instead of awaiting them. https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
```
