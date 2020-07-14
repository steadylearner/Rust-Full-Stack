# Mongoose with Docker example

First, install mongodb with these Docker commands.

```console
$docker volume create mongodbdata
$docker run -d -v mongodbdata:/data/db --name mongo -p 27017:27017 mongo
```

Then, install the npm packages with $yarn

If you want to delete the volume later, use $docker volume rm mongodbdata

## How to test it

Start the server with **$yarn dev** or **$node server.js**. Then, use these commands.

1. Register an email

```console
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
$curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
```

2. Read it

```console
$curl -X GET localhost:8000/api/email/v1/steady@learner.com
```

3. Update it

Use PATCH when you need to update only part of it. PUT when you want to update all the data.

```console
$curl -X PATCH localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
```

4. Delete it

```console
$curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
```

5. List emails

```console
$curl -X GET localhost:8000/api/email/v1
```

## How to kill the Nodemone process

Find its process id and kill it.

```
$ps aux | grep -i nodemon

steadyl+ 22681  0.2  0.9 869020 37784 ?        Sl   08:04   0:02 /home/steadylearner/.nvm/versions/node/v12.3.1/bin/node /home/steadylearner/Desktop/code/site/Rust-Full-Stack/database/node_mongoose/node_modules/.bin/nodemon server
steadyl+ 24869  0.0  0.0  21536  1008 pts/0    R+   08:17   0:00 grep --color=auto -i nodemon

$kill 22681
```

## What left

1. Include code to write errors to log file.
2. Write code to handle more errors and tests.
