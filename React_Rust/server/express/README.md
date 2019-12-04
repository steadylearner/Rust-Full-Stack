# What can you do with it

You can deploy it with aws ec2 and ecs and verify the result easily with express. Use it instead of the python app from the docker curriculum

## READ First

1. [Docker Curriculum](https://docker-curriculum.com/#docker-on-aws)
2. [aws elasticbeanstalk](https://console.aws.amazon.com/elasticbeanstalk)
3. [aws vpc](https://console.aws.amazon.com/vpc/) # Create VPC and the default vpc

There are many requirements uncommented from the curriculum. You should make vpc, subnet, IAM rules etc to make it all work.

## Payloads

1. Dockerfile
2. Dockerun.aws.json

## Commands

1. $docker build -t steadylearner/express .**
2. $docker run --name express -p 80:8000 -d steadylearner/express.**
3. $docker stop containerid && docker rm containerid
4. $docker restart containerid, $docker start containerid
5. $docker exec -it containerid bash

Refer to [How to use Docker commands](https://www.steadylearner.com/blog/read/How-to-use-Docker-commands).

## Result

```console
IMAGE                   COMMAND             CREATED             STATUS                     PORTS                  NAMES
steadylearner/express   "yarn serve"        4 minutes ago       Up 4 minutes               0.0.0.0:80->8000/tcp   express
```

## DockerHub

```console
$docker push steadylearner/express
```

Then, visit [the repository](https://hub.docker.com/repository/docker/steadylearner/express)

## AWS

Use ECS fargate instead of EC2 to make the workflow more compatible with dockerized microservices. 

Save this to ~/.bashrc.

```bash
#aws 
alias configure-ecs="ecs-cli configure --region us-east-1 --cluster docker"
alias start-ecs="ecs-cli up --keypair docker --capability-iam --size 2 --instance-type t2.micro --force"
alias fargate="ecs-cli compose up --launch-type FARGATE"
alias ecs-list="ecs-cli ps"
alias ec2="ecs-cli compose up"
alias stop-ecs="ecs-cli compose stop"
```
**$source ~/.bashrc** to use them.

### How to deploy your app with Fargate

**start-ecs** will make VPC, **subnets**, **security groups** autoamtically. Use them in **ecs-params.yml**.

```yml
version: 1
task_definition:
  task_execution_role: ecsTaskExecutionRole
  ecs_network_mode: awsvpc
  task_size:
    mem_limit: 0.5GB
    cpu_limit: 256
run_params:
  network_configuration:
    awsvpc_configuration:
      subnets:
        - "yoursubnet"
        - "yourothersubnet"
      security_groups:
        - "yoursecuritygroup"
      assign_public_ip: ENABLED
```

Then, use **$fargate**. You can verify the result with **$ecs-list**.

### How to delete everything

Use aws website or CLI. CloudFormation, ECS, EC2, VPC, subnets, security groups, Cloudformation, RDS, elasticbeanstalk, (IAM user and roles) etc you used in the previous process. 

Find them with list or describe API from aws cli. Then, Delete all.

```console
$aws cloudformation describe-stacks
$aws cloudformation delete-stack --stack-name arn:aws:cloudformation:us-east-1:627261273737:stack/amazon-ecs-cli-setup-docker/aae51b00-0469-11ea-a4ba-0e07a69c7d52
// DELETE_IN_PROGRESS
// {
//    "Stacks": []
// }

$aws ecs help
$aws ecs list-clusters
$aws ecs delete-cluster --cluster "arn:aws:ecs:us-east-1:627261273737:cluster/docker"
$aws ecs list-clusters
// {
//    "clusterArns": []
// }
$aws ec2 describe-instances
// "State": {
//    "Code": 48,
//    "Name": "terminated"
// },
$aws ec2 describe-subnets
// SubnetId
$aws ec2 delete-subt --sunet-id SubnetId
$aws ec2 describe-subnet
// {
//    "Subnets": []
// }
// Couldn't find it in the browser panel, so made a case and who works there sent me the id
$aws rds describe-db-snapshots
$aws rds delete-db-snapshot  --db-snapshot-identifier docker-final-snapshot
// Verify the result at https://console.aws.amazon.com/ec2/v2

// [Optional]
// Visit it and delete subnet and security groups except default ones. https://us-west-2.console.aws.amazon.com/vpc/
$aws ec2 describe-vpcs
// VpcId
// Delete these first before you delete vpc made from this process if it is not default.
// https://aws.amazon.com/pt/premiumsupport/knowledge-center/troubleshoot-dependency-error-delete-vpc/
// "InternetGatewayId"
// "RouteTableId"
// "RouteTableId"
// "NetworkAclId"
// "GroupId"
$aws ec2 delete-vpc --vpc-id VpcId
($aws iam list-users)
($aws iam delete-user)
```

Then, verify this https://console.aws.amazon.com/billing/home regularly.
