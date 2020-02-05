# Mongoose with Docker example

First, install mongodb with these Docker commands.

```console
$docker volume create mongodbdata
$docker run -d -v mongodbdata:/data/db --name mongo -p 27017:27017 mongo 
```

Then, install the npm packages with $yarn

## How to test it

Start the server with **$yarn dev** or **$node server.js**. Then, use these commands.

1. Register an email

```console
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
```

2. Read it

```console
$curl -X GET localhost:8000/api/email/v1
```

3. Update it

```console
$curl -X PUT localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
```

4. Delete it

```console
$curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
```

5. List emails

```console
$curl -X GET localhost:8000/api/email/v1
```
