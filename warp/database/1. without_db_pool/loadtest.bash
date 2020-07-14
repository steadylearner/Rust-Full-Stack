#!/bin/bash
# https://www.npmjs.com/package/loadtest
# $cargo run --release 
# first to make the server running

npm install -g loadtest
loadtest http://0.0.0.0:8000/api/post/v1 -t 20 -c 10 --keepalive --rps 2000

