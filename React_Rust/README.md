[How to use React with Rust Actix]: https://www.steadylearner.com/blog/read/How-to-use-React-with-Rust-Actix
[How to use Docker commands]: https://www.steadylearner.com/blog/read/How-to-use-Docker-commands
[How to use Docker with Rust]: https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust

# React Full Stack Example

React and various servers to test it. It will be improved with Blog Posts at Steadylearner.

![user-signup](/src/images/screenshot/user-signup.png)

## How to test frontend

React, Webpack, Formik, Jest, React-Testing-Library, Cypress etc.

```console
$nvm install 12.3.1
$nvm use 12.3.1
$yarn
$yarn start
```

## How to test it with server

You can compare the performance with **$./loadtest.bash**.

I would use Express normally or Restify for simple Rest JavaScript project.

If I care for speed, the choice will be among Actix, Golang, Vibora. Actix shows the performance as expected. Golang was way better than I thought. The result from Vibora was similar to the others. I think it is underated.

Rocket, Django don't seem to be adequate for many concurrent requests.

### Express

Webpack Dev server is based on Express. Therefore, prototype with it first. We will use it to learn how to deploy these web server to AWS. Only ports and frameworks will be different.

```console
./express.bash
```

1. The result form loadtest 

```console
// No errors but mean latency is problem?

INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 1769, requests per second: 354, mean latency: 578.9 ms
INFO Requests: 5636, requests per second: 774, mean latency: 786.2 ms
INFO Requests: 10162, requests per second: 906, mean latency: 1392.7 ms
INFO 
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO 
INFO Completed requests:  14669
INFO Total errors:        0
INFO Total time:          20.001914555 s
INFO Requests per second: 733
INFO Mean latency:        1012.1 ms
INFO 
INFO Percentage of the requests served within a certain time
INFO   50%      1052 ms
INFO   90%      1537 ms
INFO   95%      1629 ms
INFO   99%      1974 ms
INFO  100%      8906 ms (longest request)
```

2. Memory usage(**113 MB**)

### Warp

```console
./warp.bash
```

1. The result from loadtest

```console
// The best result considering its low memory usage compared to Golang

INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 3832, requests per second: 767, mean latency: 8.3 ms
INFO Requests: 11189, requests per second: 1473, mean latency: 5.2 ms
INFO Requests: 19382, requests per second: 1638, mean latency: 4.8 ms
INFO
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO
INFO Completed requests:  27364
INFO Total errors:        0
INFO Total time:          20.001672077 s
INFO Requests per second: 1368
INFO Mean latency:        5.5 ms
INFO
INFO Percentage of the requests served within a certain time
INFO   50%      4 ms
INFO   90%      8 ms
INFO   95%      10 ms
INFO   99%      16 ms
INFO  100%      82 ms (longest request)
```

2. Memory usage(**37.44 Mb**)

### Rocket

Use it to test you can deploy React Rocket application to AWS. Then, you should separate the project. Use **/static/<file..>** instead of current **/<file..>** in **routes/static_files.rs** to serve static files. It will help the Rocket server not to show errors from it.

```console
./rocket.bash
```

### Actix

[How to use React with Rust Actix]

```console
./actix.bash
```

1. The result from loadtest

```console
// No errors and fast with the lowest memory usage.

INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 1797, requests per second: 360, mean latency: 43.2 ms
INFO Requests: 4855, requests per second: 612, mean latency: 34.4 ms
INFO Requests: 9126, requests per second: 833, mean latency: 29.6 ms
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

2. Memory usage(**13 Mb**)

### Restify

```console
./restify.bash
```

### Golang

```console
./golang.bash
```

1. The reults from loadtest 

```console
// It is one of the fastest and no errors. It is made for this.

INFO Requests: 0, requests per second: 0, mean latency: 0 ms
INFO Requests: 3820, requests per second: 772, mean latency: 9 ms
INFO Requests: 11529, requests per second: 1541, mean latency: 5 ms
INFO Requests: 20169, requests per second: 1729, mean latency: 4.0 ms
INFO 
INFO Target URL:          http://0.0.0.0:8000/
INFO Max time (s):        20
INFO Concurrency level:   10
INFO Agent:               keepalive
INFO Requests per second: 2000
INFO 
INFO Completed requests:  27881
INFO Total errors:        0
INFO Total time:          20.002745433 s
INFO Requests per second: 1394
INFO Mean latency:        5.4 ms
INFO 
INFO Percentage of the requests served within a certain time
INFO   50%      4 ms
INFO   90%      8 ms
INFO   95%      10 ms
INFO   99%      19 ms
INFO  100%      339 ms (longest request)
```

2. Memory usage(**231 Mb**)

### Vibora

You should use **$python3.6 -m venv python** and include Vibora folder in it.

```console
./vibora.bash
```

### Django

Some Python frameworks requires you to include all the static files in /static folder and not /. So it was not easy fo make it work.

This will be the last framework to compare for a while.

You should use **$python -m venv python** and include Django folder in it.

```console
./django.bash
```

## Blog posts

1. [How to use Webpack with React](https://www.steadylearner.com/blog/read/How-to-use-Webpack-with-React)
2. [How to use Cypress with React](https://www.steadylearner.com/blog/read/How-to-use-Cypress-with-React)
3. [How to use gRPC with Rust Tonic and Postgresql database](https://www.steadylearner.com/blog/read/How-to-use-gRPC-with-Rust-Tonic-and-Postgresql-database)
4. [How to use Docker commands]
5. [How to use Docker with Rust]

## Screenshots

![user-result](/src/images/screenshot/user-result.png)
![user-list](/src/images/screenshot/user-list.png)
