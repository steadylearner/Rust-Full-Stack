# What can you do with it

You can deploy it with aws ec2 and ecs and verify the result easily with actix. Use it instead of the python app from the docker curriculum

## READ First

1. [Docker Curriculum](https://docker-curriculum.com/#docker-on-aws)
2. [aws elasticbeanstalk](https://console.aws.amazon.com/elasticbeanstalk)
3. [aws vpc](https://console.aws.amazon.com/vpc/) # Create VPC and the default vpc

There are many requirements uncommented from the curriculum. You should make vpc, subnet, IAM rules etc to make it all work.

## Payloads

1. Dockerfile
2. Dockerun.aws.json

## Commands

1. $docker build -t steadylearner/actix .**(This make a Docker image nad not containers.)
2. $docker run --name actix -p 80:8000 -d steadylearner/actix.**(You should execute it.)
3. $docker stop containerid && docker rm containerid
4. $docker restart containerid, $docker start containerid
5. $docker exec -it containerid bash

Refer to [How to use Docker commands](https://www.steadylearner.com/blog/read/How-to-use-Docker-commands).

## Result

```console
IMAGE                   COMMAND             CREATED             STATUS                     PORTS                  NAMES
steadylearner/actix   "yarn serve"        4 minutes ago       Up 4 minutes               0.0.0.0:80->8000/tcp   actix
```

## DockerHub

```console
$docker push steadylearner/actix
```

Then, visit [the repository](https://hub.docker.com/repository/docker/steadylearner/actix)

## AWS

Rust projects becomes very large in total size. So it is not worth using it in production with processes. You can use it if you have a better machine with high internet speed.
