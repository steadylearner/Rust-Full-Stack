# The result from loadtest

There were errors. Compare it to Golang, Vibora, Actix that were without erros and very fast.

```console
INFO Requests: 1086, requests per second: 216, mean latency: 133.9 ms
INFO Requests: 2548, requests per second: 295, mean latency: 582.9 ms
INFO Requests: 4011, requests per second: 293, mean latency: 1354.5 ms
INFO Errors: 115, accumulated errors: 115, 2.9% of total requests
INFO
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  5589
INFO Total errors:        492
INFO Total time:          20.006069541 s
INFO Requests per second: 279
INFO Mean latency:        1037.8 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      747 ms
INFO   90%      2419 ms
INFO   95%      2872 ms
INFO   99%      4934 ms
INFO  100%      7192 ms (longest request)
INFO
INFO  100%      7192 ms (longest request)
INFO
INFO   404:   492 errors
```
