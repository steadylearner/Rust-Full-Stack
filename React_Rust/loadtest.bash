#!/bin/bash
# https://www.npmjs.com/package/loadtest

# When you want to see the memory usage, use the command below.
# $ps -eo size,pid,user,command --sort -size | awk '{ hr=$1/1024; printf("%13.2f Mb ",hr) } { for ( x=4 ; x<=NF ; x++ ) { printf("%s ",$x) } print "" }' | cut -d "" -f2 | cut -d "-" -f1

# Use it with grep if you want. For example,
# $ps -eo size,pid,user,command --sort -size | awk '{ hr=$1/1024; printf("%13.2f Mb ",hr) } { for ( x=4 ; x<=NF ; x++ ) { printf("%s ",$x) } print "" }' | cut -d "" -f2 | cut -d "-" -f1 | grep "cargo"

npm install -g loadtest
loadtest http://0.0.0.0:8000/ -t 20 -c 10 --keepalive --rps 2000

