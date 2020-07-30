# The result from loadtest

No errors and reasonably fast. It was definitely better to reuse connection(More than 22 times better). It seems that the reason of errors was because the server had to accecpt every request to connect to database.

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 2466, requests per second: 498, mean latency: 86.1 ms
INFO Requests: 5477, requests per second: 603, mean latency: 476.4 ms
INFO Requests: 8584, requests per second: 620, mean latency: 821.9 ms
INFO
INFO Target URL:          http://0.0.0.0:8000/api/post/v1
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  11310
INFO Total errors:        0
INFO Total time:          20.003117999999997 s
INFO Requests per second: 565
INFO Mean latency:        745 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      177 ms
INFO   90%      763 ms
INFO   95%      5712 ms
INFO   99%      8309 ms
INFO  100%      9276 ms (longest request)
```
