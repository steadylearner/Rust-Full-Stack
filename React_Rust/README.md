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

### Restify

```console
./restify.bash
```

### Golang

```console
./golang.bash
```

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
