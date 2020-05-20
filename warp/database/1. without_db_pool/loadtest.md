# The result from loadtest

There are errors but few.

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 110, requests per second: 22, mean latency: 2097.6 ms
INFO Requests: 227, requests per second: 23, mean latency: 4056.5 ms
INFO Requests: 370, requests per second: 29, mean latency: 4792.6 ms
INFO
INFO Target URL:          http://0.0.0.0:8000/api/post/v1
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  508
INFO Total errors:        2
INFO Total time:          20.027096512 s
INFO Requests per second: 25
INFO Mean latency:        5206.3 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      4781 ms
INFO   90%      9924 ms
INFO   95%      10552 ms
INFO   99%      11476 ms
INFO  100%      19084 ms (longest request)
INFO
INFO  100%      19084 ms (longest request)
INFO
INFO    -1:   2 errors
```
