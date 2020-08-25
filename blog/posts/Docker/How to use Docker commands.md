<!--
    Post{
        subtitle: "Learn how to use Docker and upload its images to DockerHub"
        image: "posts/web/docker.png",
        image_decription: "Image from the official website",
        tags: "How use Docker code",
    }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Docker Website]: https://docs.docker.com/get-started
[How to install Docker]: https://www.google.com/search?q=how+to+install+docker
[How to deploy a container with Docker]: https://thenewstack.io/how-to-deploy-a-container-with-docker/
[Docker Curriculum]: https://docker-curriculum.com/
[Docker Hub]: https://hub.docker.com/
[Docker lifecycle]: (https://medium.com/@nagarwal/lifecycle-of-docker-container-d2da9f85959).
[AWS]: https://aws.amazon.com
[Elastic Beanstalk]: https://aws.amazon.com/pt/elasticbeanstalk/
[ECS]: https://aws.amazon.com/ecs/
[Cloud​Formation]: https://aws.amazon.com/pt/cloudformation/
[Yarn]: https://yarnpkg.com/lang/en/
[Express]: https://expressjs.com/

<!-- / -->

<!-- Steadylearner Post -->

[Steadylearner Blog]: https://www.steadylearner.com/blog

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In this post, we will learn how to use Docker commands. We will make a web app inside a Docker container and turn it into an Docker image and learn how to upload it to [Docker Hub].

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to install Docker]
2. [Docker Website], [Docker Curriculum]
3. [How to deploy a container with Docker], [Docker lifecycle]

---

You should install Docker first if you haven't yet. Simply type **$docker** in your machine and this would show you how to install it. Otherwise, search how to install it in your browser.

This post is the summary of [Docker Website], [Docker Curriculum] etc. I hope you to read them first but you don't have to spend much time for them. We will learn how to deploy a web app and microservices to [AWS] with other [Steadylearner Blog] posts.

<br />

<h2 class="blue">Table of Contents</h2>

1. Confirm installation with Nginx
2. Set up your development environment with Docker
3. How to move your local files and folders to docker contaienrs
4. How to use the web framework with docker containers
5. How to modify the network ports of docker images
6. Docker Images and Containers
7. How to upload your Docker images to DockerHub
8. Conclusion

---

<br />

## 1. Confirm installation with Nginx

I hope you could install Docker. Before we learn how each Docker command work, we will use these to verify it can show you some results in your machine or not.

Use them in your CLI.

```console
$docker search nginx
$docker pull nginx
$docker run --name nginx-webserver -p 80:80 nginx
```

Then, visit [localhost](http://localhost) and it will show this in your browser.

```console
Welcome to nginx!

If you see this page, the nginx web server is successfully installed and working. Further configuration is required.

For online documentation and support please refer to nginx.org.
Commercial support is available at nginx.com.

Thank you for using nginx.
```

Find you just need a few commands to use Docker. You could also start a Docker container with a specific name and execute bash comamnds with these.

```console
$docker run --name nginx-webserver -p 80:80 -d nginx
$docker exec -it CONTAINER_ID bash
```

<br />

## 2. Set up your development environment with Docker

In this part, we will learn how to set up a default Docker image with Ubuntu you can reuse later. If you use another OS, please use that instead and refer to this part.

Start with **pull** commands to download the official ubuntu image from [Docker Hub]. If you are new to Docker Hub, you can compare this similar to GitHub for its repositories.

```console
$docker pull ubuntu
```

Then, make a container in your machine and use sh or bash commands in it with this to download minimal softwares.

```console
$docker run -it ubuntu sh
```

Start by installing **CURL** to download other programs.

```console
$apt-get update
$apt-get install curl
$curl https://www.steadylearner.com
```

If you are out of the container, you can start it again with this.

```console
$docker exec -it CONTAINER_ID bash
```

You can find the CONTINAER_ID with **docker ps -a**. This is the command you will use many times and show some useul meta datas of Docker containers.

We will make a simple Node "Hello, World" web app example for this post. So we will need to set up the Node development environment first. Follow these processes if you want to use the same project for this post.

You can use [$docker run -d steadylearner/ubuntu_node](https://cloud.docker.com/u/steadylearner/repository/docker/steadylearner/ubuntu_node) instead also.

You should be inside your docker container to use them.

<details>
  <summary class="red-white font-normal hover cursor-pointer transition-half">Node, NPM, Yarn</summary>

```console
curl -sL https://deb.nodesource.com/setup_12.x | bash
```

will show this.

```console
## Run `sudo apt-get install -y nodejs` to install Node.js 12.x and npm
## You may also need development tools to build native addons:
    sudo apt-get install gcc g++ make
## To install the Yarn package manager, run:
    curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add -
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list
    sudo apt-get update && sudo apt-get install yarn
```

You should use commands without sudo.

```console
apt-get install gcc g++ make
curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list

apt-get update && apt-get install yarn
```

Follow those commmands and install them all.

Test node work with this.

```console
$node
$console.log("Hello from www.steadylearner.com");
```

</details>

<details>
  <summary class="red-white font-normal hover cursor-pointer transition-half">Vim</summary>

Use this command with --assume-yes or -y to skip install relevant questions.

```console
apt install --assume-yes vim
```

It will install Vim text editor. Then use this command to use it.

```console
$vim hello.js
```

Then, edit your hello.js file with this and **:wq** to save and quit from the Vim.

```js
// hello.js
console.log("Hello from www.steadylearner.com");
```

Use this command to verify that Node is installed well.

```console
$node hello.js
// Hello from www.steadylearner.com
```

</details>

<details>
  <summary class="red-white font-normal hover cursor-pointer transition-half">Git</summary>

```console
$apt-get --assume-yes git-core
```

It will install git and verify it installed and its version with this.

```console
$git --version
```

Then, get your GitHub user name and email from your local machine.

```console
$git config --get user.name
$git config --get user.email
```

Use them in the docker container to use Git inside of it.

```console
$git config --global user.name yourname
$git config --global user.name youremail
```

Use the same command(--get) before to verify them in your Docker container.

Test git clone work to download files from your previous GitHub repositories.

For example, clone steadylearner/docker-examples repository with this.

```console
$git clone https://github.com/steadylearner/docker-examples.git
```

</details>

I hope you could install what you think necessary in your Docker container.

You can skip this yarn relevant part and use default npm commands instead. Otherwise, read [this post](https://linuxize.com/post/how-to-install-yarn-on-ubuntu-18-04/) for more information.

Verify the [Yarn] version first in it.

```console
$yarn -v
```

It will show the version of your Yarn.

Then, use these commands to use Node project.

```console
$cd /home
$mkdir node && cd node
$yarn init
$yarn add chalk
```

Test NPM or Yarn work with NPM modules with these.

```js
// Start with $node in your console and use each command.

const chalk = require("chalk");
const blue = chalk.blue;
const hello = blue("Hello from www.steadylearner.com");
console.log(hello);
```

It should have shown colored **Hello from www.steadylearner.com** message in your console.

We verified that NPM packages work in your docker container with this.

If you want, make alias for this directory also similar to this.

```console
$vim ~/.bashrc
```

Type this and :wq to save and quit.

```bash
alias work="cd /home/node"
```

**$source ~/.bashrc** and you can use your node project with **$work** whenever you want. You can also define WORKDIR later with **Dockerfile** or **docker-compsose.yml** instead for the same purpose.

There will be many Docker containers in your machine. You can remove unnecessary ones with these commands.

**1.** List and remove previous Docker containers.

```console
$docker ps -a
```

It will show the list of the instances you executed before. 

**2.** Remove what you won't need.

```console
$docker stop containerid
$docker rm containerid
```

or

```console
$docker rm containerid -f
```

<br />

## 3. How to move your local files and folders to docker contaienrs

We can use git commands to donwload files from the GitHub to your containers. You can also use Docker commands to move local files and folders to your Docker conatiners and vice versa.

Refer to these examples or **docker cp --help**.

**1.** Files

```console
$docker cp from_localhost.txt containerid:/from_localhost.txt
$docker cp containerid:/from_docker from_docker.txt
```

**2.** Folders

```console
$docker cp from_localhost containerid:/from_localhost
$docker cp containerid:/from_localhost from_localhost
```

<br />

## 4. How to use web frameworks with docker containers

We installed Node relevant softwares for this part. If you use web frameworks from other languages, please refer to the workflow of this part only.

<details>
  <summary class="red-white font-normal hover cursor-pointer transition-half">Express</summary>

Install the dependencies we will use inside the docker container with this.

```console
$yarn add express chalk
```

Then, we will build "Hello, World!" app with the JavaScript code below.

```js
// server.js
const express = require('express')
const chalk = require("chalk");

const app = express()
const port = 3000

app.get('/', (req, res) => res.send('Hello, World!'))

const blue = chalk.blue
const target = blue(`http://localhost:${port}`)

app.listen(port, () => console.log(`Express Server ready at ${target}`))
```

Then, **$node server.js** will show this message.

```console
Express Server ready at http://localhost:3000
```

But **$curl http://localhost:3000** or visiting it in your browser won't work yet.

Each container has its own ip to network. We should inspect the docker container with **$docker inspect CONTAINER_ID > inspect.txt** to extract the information of the container.

You can find its local ip at the end of inspect.txt and will be similar to **172.17.0.2**. You can also make getIP.js and **$node getIP.js** to save your time.

```js
const fs = require('fs')

const filename = "inspect.txt";

fs.readFile(filename, 'utf8', function(err, data) {
  if (err) throw err;

  // console.log(`Read ${filename}`);
  const dataObject = JSON.parse(data);

  // console.log(payload);
  // console.log(typeof payload);

  const ip = dataObject[0].NetworkSettings.IPAddress;
  console.log(`IP is ${ip}`);
});
```

You can also use the [docker inspect command](https://docs.docker.com/engine/reference/commandline/inspect/).

Test the ip with $curl http://172.17.0.2:3000/ or verify it with your browser.

If you could see this message, you can see that you can develop web apps in your local machine with Docker.

```console
Hello, World!
```

</details>

<br />

## 5. How to modify the network ports of docker images

In the previous part, we had to find the network port for the web framework to visit it. Instead, [you can start with your custom port](https://www.google.com/search?&q=how+to+assign+port+for+docker+container).

```console
$docker run -it --name ubuntu_node -p 80:80 ubuntu
```

You can also use it with **-d** to [make the container run in background.](https://docs.docker.com/engine/reference/run/).

```console
docker run -d --name ubuntu_node -p 80:80 ubuntu:latest
```

Refer to this to find what happens here better.

'By default, the port on the host(container) is mapped to 0.0.0.0, which means all IP addresses. You can specify a particular IP when you define the port mapping, for example, -p 127.0.0.1:80:80'

<br />

## 6. Docker Images and Containers

You may be confused with the difference between Docker container and image. You can simply think the images are classes and containers are the instances you are using in your machine.

1. You can pull or run(pull and start) images and it will make docker containers from it in your local machine.

2. You can edit files in your containers with **$docker exec -it containername bash**.

3. You can make images from the containers with **$docker commit containername YourDockerHub/image && docker push account/image**.

You can also start with Dockerfile instead of **1.** and **2.** and commit your docker images also. We will learn that in another [Steadylearner Blog] with [Elastic Beanstalk].

<br />

## 7. How to upload your Docker images to DockerHub

We will learn how to [create a repository first at Dockerhub](https://cloud.docker.com/repository/create) with the example we made.

Login first with this command.

```console
$docker login
```

Then, use [$docker commit](https://www.scalyr.com/blog/create-docker-image).

```console
$docker commit ubuntu_node
```

Then, verify the image was made from the ubuntu_node container with this.

```console
$docker images
```

Give it tag(name).

```console
$docker tag imageid steadylearner/ubuntu_node
```

You could execute this command instead of them.

```console
$docker commit ubuntu_node steadylearner/ubuntu_node
```

Then, you can push your docker image to Docker Hub with this.

```console
$docker push steadylearner/ubuntu_node // yourusername/image
```

Wait for uploading process to complete and use this.

```console
$docker run -it steadylearner/ubuntu_node bash
```

Then, follow the same process you used before to edit it if you want.

You can restart the containers with this if it stopped.

```console
$docker restart containerid
$docker exec -it containerid bash
```

You can remove container made from steadylearner/ubuntu_node image or yours with this.

```console
$docker stop ubuntu_node
$docker container rm ubuntu_node
$docker image rm ubuntu
```

You can also rename the container.

```console
$docker container rename randomname ubuntu_node
```

Use yours instead of ubuntu_node or steadylearner/ubuntu_node.

If you modify the project, use the commands similar to this.

```console
$docker commit ubuntu_node steadylearner/ubuntu_node
```

Or with a commit message.

```console
$docker commit --message "Test message and will be similar to github -m option" ubuntu_node steadylearner/ubuntu_node
```

Then, use this to push the image made from it to DockerHub.

```console
$docker push steadylearner/ubuntu_node
```

and use this 

```console
$docker run -it steadylearner/ubuntu_node bash
```

or this to verify the result.

```console
$docker history steadylearner/ubuntu_node
```

<br />

## 8. Conclusion

I hope you made it all work. We could install Docker and made it work with Nginx. Then, we made s Docker container and image and uploaded them to [DockerHub].

In the later [Steadylearner Blog], we will learn how to deploy the web app with [Elastic Beanstalk] from [AWS] and Dockerfile. We will also learn how to deploy micro services with [ECS], [Cloud​Formation], **docker-compose.yml** etc.

There are many things to learn. But, everything will be easy with examples.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need **a Full Stack Rust Developer** who can deploy the projects with Docker, AWS etc? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others**.

If you want more you can refer to these commands. You can use the id or name of containers for them.

### Logs of the container

```console
$docker logs containerid | name
```

### History of the image

```console
#docker history steadylearner/ubuntu_node
```

### Remove unused images

```console
$docker images
$docker image rm dockerimagename or docker rmi
```

### Rename the container

```console
$docker rename randomname whatyouwant
```

### Pause and unpause the containers

```console
$docker pause containerid | name
$docker ps -a
$docker unpuase containerid | name
$docker ps -a
```

### Start and stop them

```console
$docker stop containerid | name
$docker ps -a
$docker start containerid | name
$docker ps -a
```

### Remove containers

```console
$docker container rm containerid | name
```
