# How to test it

Use this command with redis and without the data from redis($redis-cli && del steadylearner).

```console
curl -w "@curl-format.txt" -o /dev/null -s "http://0.0.0.0:8000/api/user/v1/steadylearner"
```

## Result

Redis is faster?
