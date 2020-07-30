# The result of loadtest

1. The reason of the errors is from **tokio-runtime-worker**.

```console
thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an
`Err` value: Error(Client, hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 24, kind: Other, message: "Too many open files" })))', src/libcore/result.rs:1189:5
```

2. Log

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 51, requests per second: 10, mean latency: 2448.9 ms
INFO Requests: 157, requests per second: 21, mean latency: 5647.1 ms
INFO Errors: 54, accumulated errors: 54, 34.4% of total requests
INFO Requests: 238, requests per second: 16, mean latency: 9651.3 ms
INFO Errors: 22, accumulated errors: 76, 31.9% of total requests
INFO
INFO Target URL:          http://0.0.0.0:8000/api/user/v1/steadylearner
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  307
INFO Total errors:        93
INFO Total time:          20.091578985 s
INFO Requests per second: 15
INFO Mean latency:        7940 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      7338 ms
INFO   90%      14110 ms
INFO   95%      16510 ms
INFO   99%      17084 ms
INFO  100%      18038 ms (longest request)
INFO
INFO  100%      18038 ms (longest request)
INFO
INFO    -1:   93 errors
```
