# The result from loadtest

It is very fast and no errors. It is sad that it is dead.

```console
INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 1797, requests per second: 360, mean latency: 43.2 ms
INFO Requests: 4855, requests per second: 612, mean latency: 34.4 ms
INFO Requests: 9026, requests per second: 833, mean latency: 29.6 ms
INFO
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  13300
INFO Total errors:        0
INFO Total time:          20.005901511 s
INFO Requests per second: 665
INFO Mean latency:        32.4 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      26 ms
INFO   90%      59 ms
INFO   95%      64 ms
INFO   99%      75 ms
INFO  100%      110 ms (longest request)
```
