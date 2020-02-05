# Rust gRPC microservices with Warp and Tonic

This is to show **it is possible to make microservies with Rust**. We can deploy them to AWS with Docker also. In the near future, Mongodb and REST end points from **2.express_mongoose** for them will be also included.

## How to test it locally

I will assume that you are already familar with Docker and other technologies used here. If not, please read these blog posts.

1. [How to use Docker commands](https://www.steadylearner.com/blog/read/How-to-use-Docker-commands)
2. [How to use Docker with Rust](https://www.steadylearner.com/blog/read/How-to-use-Docker-with-Rust)
3. [Rust blog posts](https://www.steadylearner.com/blog/search/Rust)

First, **$cd local** and you will see that there are **tonic_server** and **warp_client** directory.

The role of **tonic_server** is to serve data with gRPC API. It gets data from Postgresql database and Redis cache for it.

**warp_client** is a server but it is client of gRPC server. So it is named **warp_client** because the entire purpose of this project is to show **gRPC full stack server and client example written in Rust**.

The Warp framework handles routing, HTTP REST API, Testing, and potentially login etc later.

To test all this work, you should install Postgresql and Redis first. I would use docker for it. So I let the commands for them here to help you.

```console
// 1. Install Postgresql with volume to make data persist.
$docker volume create postgresqldata
$docker run -d -v postgresql:/data/db --name postgresql -p 5432:5432 postgresql
// 2. Install Redis
$docker run -d --name redis -p 6379:6379 redis
```

You can remove the volume later with **$docker volume rm postgresqldata** or **$docker volume prune**.

Otherwise, you can also use **--network="host"** option instead of -p **port:port** part.

Make some records at Postgresql database while you refer to these commands.

```sql
-- http://www.postgresqltutorial.com/postgresql-uuid/
-- CREATE DATABASE grpc OWNER you;
-- \c grpc;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- SELECT uuid_generate_v4();

CREATE TABLE users (
    -- use VARCHAR instead of uuid because of the type problem.
    -- Should find the better way and modify the code later if necessary.
    -- It is also easy to test end points with this.
    id VARCHAR(255) PRIMARY KEY DEFAULT uuid_generate_v4 (),
    first_name VARCHAR(60) NOT NULL,
    last_name VARCHAR(60) NOT NULL,
    date_of_birth Date NOT NULL
);

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('steadylearner', 'steady', 'learner', '2019-01-01');

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayisblackfriday', 'mybirthdayis', 'blackfriday', '2019-11-25');

INSERT INTO users
    (id, first_name, last_name, date_of_birth)
VALUES
    ('mybirthdayisnotblackfriday', 'mybirthdayis', 'notblackfriday', '2019-11-26');
```

Then, **$cargo run --release** in **tonic_server** and **warp_client** in separate kernel to make both server work before you test the end points from **warp_client**.

## How to test it with CURL

Refer to these examples to use CURL to test the end points form warp_client. You can also compare them with tests in **warp_client/src/tests** folder.

The main purpose of these end points is to return **hasehd full_name** datas from first_name and last_name given by users.

1. List users

```console
$curl 0.0.0.0:8000/api/user/v1
```

2. Request data of a user

```console
$curl 0.0.0.0:8000/api/user/v1/steadylearner
```

3. Create a user data

```console
$curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json" -d '{ "first_name": "steady", "last_name": "learner", "date_of_birth": "2019-01-01" }'
```

4. Update a user

```console
$curl -X PUT 0.0.0.0:8000/api/user/v1/steadylearner -H "authorization: user" -H "Content-Type: application/json" -d '{ "first_name": "fullstack rust", "last_name": "developer", "date_of_birth": "2019-01-01" }'

// It becomes {
//     id: "steadylearner",
//     first_name: "fullstack rust",
//     last_name: "developer",
//     date_of_birth: "2019-01-01",
// },

$curl -X PUT 0.0.0.0:8000/api/user/v1/steadylearner -H "authorization: user" -H "Content-Type: application/json" -d '{ "first_name": "steady", "last_name": "learner", "date_of_birth": "2019-01-01" }'

// It becomes {
//     id: "steadylearner",
//     first_name: "steady",
//     last_name: "learner",
//     date_of_birth: "2019-01-01",
// },
```

5. Delete a user

```console
$curl -X DELETE -H "authorization: steadylearner" 0.0.0.0:8000/api/user/v1/f2bd8139-5044-4526-89b8-1981d6220b4

// No more records in Postgresql.
// \c grpc;
// $SELECT * FROM users WHERE id = 'f2bd8139-5044-4526-89b8-1981d6220b4';
```
