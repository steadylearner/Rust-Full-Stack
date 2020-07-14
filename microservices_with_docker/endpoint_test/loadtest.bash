#!/bin/bash
# https://www.npmjs.com/package/loadtest

# sudo apt install npm
sudo npm install -g loadtest
# Test the GET API
loadtest http://0.0.0.0:8000/api/user/v1/steadylearner -t 20 -c 10 --keepalive --rps 2000
# thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Error(Client, hyper::Error(Connect, ConnectError("tcp connect error", Os { code: 24, kind: Other, message: "Too many open files" })))', src/libcore/result.rs:1189:5

 
