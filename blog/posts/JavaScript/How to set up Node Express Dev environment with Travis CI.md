<!--
    Post{
        subtitle: "Learn how to set up simple Node Express project"
        image: "code/Express.png",
        image_decription: "Image from the website",
        tags: "How Node Express Travis",
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
[Node]: https://nodejs.org/en/
[NPM]: https://www.npmjs.com/
[Yarn]: https://yarnpkg.com/lang/en/
[Express]: https://expressjs.com/
[ESLint]: https://eslint.org/
[Jest]: https://jestjs.io/
[Tape]: https://github.com/substack/tape
[Supertest]: https://github.com/visionmedia/supertest
[Travis CI]: https://travis-ci.org/
[node-postgres]: https://node-postgres.com/
[gRPC Node]: https://github.com/grpc/grpc-node
[travis-ci]: https://github.com/steadylearner/travis-ci

[chalk]: [https://github.com/chalk/chalk]

<!-- / -->

<!-- Steadylearner Post -->

[Steadylearner Blog]: https://www.steadylearner.com/blog
[How to make es5 JavaScript code from es6+ with Babel]: https://www.steadylearner.com/blog/read/How-to-make-es5-JavaScript-code-from-es6+-with-Babel
[How to use Docker commands]: https://www.steadylearner.com/blog/read/How-to-use-Docker-commands
[How to automatically commit files to GitHub with Python]: https://www.steadylearner.com/blog/read/How-to-automatically-commit-files-to-GitHub-with-Python

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!--  -->

In this post, we will learn how to set up Node [Express] development environment.  We will start with the route that returns "Hello, World" to simplify the process.

You will have the Express project with [ESLint], [Jest], [Tape], [Supertest], [Travis CI] etc after you read this post.

In the following [Steadylearner Blog] posts, we will improve it to make [gRPC Node] micro services with [Postgresql][node-postgres]. Then, we will learn how to deploy it to [Elastic Beanstalk], [ECS], [Cloud​Formation] etc from [AWS].

The purpose of this post is to make their starting point you can easily refer to later.

We won't include [How to make es5 JavaScript code from es6+ with Babel]. You can use bundlers or read the post and combine it with the code from this post if you want es6+ features.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [Node], [NPM], [Yarn]
2. [How to use Docker commands]
3. [Express]

---

I hope you have already used [Node], [NPM] and [Yarn] before and installed them in your machine. This is the following post of [How to use Docker commands]. It will help you to install [Node], [NPM] also if you haven't yet.

If you haven't used the [Express] before, read the documentation of it and spend a few hours.

<br />

<h2 class="blue">Table of Contents</h2>

1. Make Express "Hello, World" app
2. Supertest and Tape to test the end point
3. Organize the project with ESLint
4. Jest
5. Travis CI
6. Commit.py to automate the GitHub commit process
7. **Conclusion**

---

<br />

## 1. Make Express "Hello, World" app

We will start by making a simple Express "Hello, World" app. Make a directory first.

```console
$mkdir travis-ci
```

Our project will be integrated with [Travis CI] at the end of this post. Therefore, use **travis-ci** for the directory name to find it easy later. [Travis CI] requires you to make a GitHub repository first. You can make a [travis-ci] or eles first if you save your time later.

Then, we will install the dependencies with Yarn. Save the [package.json](https://github.com/steadylearner/travis-ci/blob/master/package.json) in your folder and **yarn** to install it.

```json
{
  "name": "express-example-with-travis-ci",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT",
  "dependencies": {
    "body-parser": "^1.19.0",
    "chalk": "^2.4.2",
    "dotenv": "^8.1.0",
    "express": "^4.17.1"
  },
  "devDependencies": {
    "eslint": "^6.5.1",
    "eslint-plugin-jest": "^22.17.0",
    "eslint-plugin-node": "^10.0.0",
    "jest": "^24.9.0",
    "supertest": "^4.0.2",
    "tape": "^4.11.0"
  },
  "scripts": {
    "serve": "node index.js",
    "eslint-init": "node_modules/.bin/eslint --init",
    "pretest": "eslint .",
    "fix": "yarn pretest --fix",
    "test": "jest",
    "test:watch": "yarn test --watch",
    "test:tape": "node tests/index.js"
  }
}
```

We will write a few files to return "Hello, World" at **/hello** route. Start with **index.js** that will be the entry point of your project to start your app.

```js
const chalk = require("chalk");
const app = require("./server");

const port = 3000;

app.listen(port, () => {
  const blue = chalk.blue;
  const target = blue(`http://localhost:${port}`);
  console.log(`🚀 Server ready at ${target}`);
});
```

We use chalk and separate **const target = blue(`http://localhost:${port}`);** to decorate the server log. You can test it later with **$yarn serve**.


Then, we will build **server.js** used in the **index.js** we made.

```js
// https://www.freecodecamp.org/news/how-to-write-a-production-ready-node-and-express-app-f214f0b17d8c/
const express = require("express");
const bodyParser = require("body-parser");

const app = express();

const { hello } = require("./routes");

app.use(bodyParser.urlencoded({ extended: true }));
app.use(bodyParser.json());

// https://expressjs.com/en/guide/routing.html
app.use("/hello", hello);

module.exports = app;
```

Most of your important Express codes will be here. I hope you read [Express] documentation for them and search on your own. Then, what left will be just include more routes similar to **app.use("/hello", hello);**.

You can see **hello route** is imported with **const { hello } = require("./routes");**. We will build routes directory first following [the instruction for the routes](https://expressjs.com/en/guide/routing.html).

```console
$mkdir routes
```

Then, make **hello.js** to return "hello" response for /hello route and **index.js** to export it.

```js
const express = require("express");
const router = express.Router();

router.get("/", async (req, res) => {
  console.log("Get request to /hello");
  console.log(req.headers);

  res.send("hello");
});

module.exports = router;
```

<br />

```js
// index.js
const hello = require("./hello");

module.exports = {
  hello,
};
```

**hello.js** for **hello route** an

Everything is read for your **Express** app to work. **yarn serve** and you will see **Server ready at http://localhost:3000**.

Use CURL or browser to manually test http://localhost:3000/hello route work or not.

```console
$curl http://localhost:3000/hello
// hello
```

I hope you could make it. You could use monolithic index.js or server.js instead. But, having this structure will help you test and organize your project better.

<br />

## 2. Include Supertest and Tape to test the end point

We made a very simple Express app before. We will include [Tape] and [Supertest] relevant codes to test the **/hello** end point.

Start by making **tests** folder with **$mkdir tests**. Then, make **index.js** similar to this.

```js
const test = require("tape");
const supertest = require("supertest");

const app = require("../server");
const request = supertest(app);

const assert = require('assert');
const chalk = require("chalk");

test("GET /hello with promise", done => {
  request
    .get('/hello')
    .expect(200)
    .then(response => {
      const blue = chalk.blue;
      const msg = blue("Should return 200 OK with 'hello'");

      try {
        assert.strictEqual(response.text, "hello");
      } catch (e) {
        console.log(e);
        done.fail(msg);
      }

      done.pass(msg);
      done.end();
    });
});

test("GET /hello with async", async done => {

  const blue = chalk.blue;
  const msg = blue("Should return 200 OK with 'hello'");

  try {
    const req = await request
      .get('/hello')
      .expect(200);
    assert.strictEqual(req.text, "hello");
  } catch(e) {
    console.log(e);
    done.fail(msg);
  }

  done.pass(msg);
  done.end();
});
```

I hope you already read the documenation for [Tape] and [Supertest]. 

The important parts here will be **.get**, **expect** and **assert.strictEqual** relevant codes. We convert what we did at the end of the previous part to the JavaScript code here with them.

You can also verify that we can use either **Promise** or **async await** syntax with them. The first example I could find was with Promise. I made **async await** example also for this post. It won't take more than a minute to convert code between them.

Use what you want.

Then, test it with **yarn test:tape** and it will show the message similar to this.

```console
# [GET] /hello with promise
ok 1 Should return 200 OK with 'hello'
# [GET] /hello with async
ok 2 Should return 200 OK with 'hello'
```

<br />

## 3. Organize the project with ESLint

We made a few JavaScript files and verified them work. We will include [ESLint] to find some potential bugs and organize code better.

Make a **.eslintrc.json** similar to this.

```json
{
    "env": {
        "browser": true,
        "commonjs": true,
        "es6": true,
        "jest/globals": true
    },
    "extends": [
        "eslint:recommended",
        "plugin:node/recommended",
        "plugin:jest/recommended"
    ],
    "plugins": [
        "jest"
    ],
    "globals": {
        "Atomics": "readonly",
        "SharedArrayBuffer": "readonly"
    },
    "parserOptions": {
        "ecmaVersion": 2020
    },
    "rules": {
        "indent": [
            "error",
            "tab"
        ],
        "semi": [
            "error",
            "always"
        ],
        "node/exports-style": [
            "error",
            "module.exports"
        ],
        "node/file-extension-in-import": [
            "error",
            "always"
        ],
        "node/prefer-global/buffer": [
            "error",
            "always"
        ],
        "node/prefer-global/console": [
            "error",
            "always"
        ],
        "node/prefer-global/process": [
            "error",
            "always"
        ],
        "node/prefer-global/url-search-params": [
            "error",
            "always"
        ],
        "node/prefer-global/url": [
            "error",
            "always"
        ],
        "node/no-unpublished-require": [
            "error",
            {
                "allowModules": [
                    "tape",
                    "supertest"
                ]
            }
        ],
        "node/prefer-promises/dns": "error",
        "node/prefer-promises/fs": "error",
        "jest/no-disabled-tests": "warn",
        "jest/no-focused-tests": "error",
        "jest/no-identical-title": "error",
        "jest/prefer-to-have-length": "warn",
        "jest/valid-expect": "error"
    }
}
```

Most of them are from their relevant official documenations. We used Node, Tape and Supertest and will include Jest. Refer to it and include more with [ESLint] if you want.

You can see this part in **package.json** relevant to ESLint.

```json
  "scripts": {
    "eslint-init": "node_modules/.bin/eslint --init",
    "pretest": "eslint .",
    "fix": "yarn pretest --fix",
    "test": "jest",
  }
```

**pretest** will be used before **test** to lint files before you test them. You will mostly want to use **$yarn fix** if you want to ESLint automatically fix your codes.

You can also make [.eslintignore](https://eslint.org/docs/user-guide/configuring#eslintignore) files if you want to exclude some files from it. Test **$yarn fix** in your CLI and lint your project.

<br />

## 4. Jest

In the previous part for **Tape** and **Supertest** we already made code to test the end point /hello. But, what if we want to test other features? So we will include Jest for that.

We will first set up the proejct with **jest.config.js**

```js
module.exports = {
  verbose: true,
};
```

Then, build **__tests__** folder with **$mkdir __tests** and include **hello.js** to test [Jest] work.

```js
test('Testing to see if Jest works', () => {
	const message = "Hello from www.steadylearner.com";
	expect(message).toBe(message);
});
```

Use **$yarn test** and will show the message similar to this.

```console
$eslint .
$jest
  __tests__/hello.js
```

We verified **Jest** works. Make **express.js** in **__tests__ folder similar to this.

```js
const app = require('../server');

const supertest = require('supertest');
const request = supertest(app);

describe("Test express with jest and supertest", () => {
  test("[GET] /hello ", async done => {
    const response = await request
      .get('/hello');

    expect(response.status).toBe(200);
    expect(response.text).toBe('hello');

    done();
  });
});
```

You can compare it with the **tests/index.js** we made. Then, test Jest again with **$yarn test**. You can see that **$yarn test:tape** is faster for the same purpose to test end points. Therefore, we will let **Tape** and **Supertest** example for local testing purpose.

You can use Jest only if your machine is fast enough.

<br />

## 5. Travis CI

For we made a project and tested it in our local machine, we can integrate it with CI/CD softwares such as [Travis CI]

If you haven't account for it, please make it first. Then, read [its documenation for Node js](https://docs.travis-ci.com/user/languages/javascript-with-nodejs). Refer to [this blog post](https://dev.to/nedsoft/ci-cd-with-travisci-and-coveralls-in-node-express-api-2i55) also to follow the process.

I hope you already made [the Github repostiroy][travis-ci] to test this and activate your repository at [Travis CI]. You can refer to these.

```console
https://travis-ci.org/youraccount/repository
https://travis-ci.org/steadylearner/travis-ci
```

Then, make **.travis.yml** similar to this. We used yarn here so it became very simple because the author of [Travis CI] worked instead of you. I let some explicit commands here even though some of them are defaults.

```yml
language: node_js
node_js:
  - "stable"
install: yarn
cache: yarn
script: yarn test
```

Include the code similar to this in your README.md and commit your GitHub project.

```md
[![Build Status](https://travis-ci.org/steadylearner/travis-ci.svg?branch=master)](https://travis-ci.org/steadylearner/travis-ci)
```

Wait for a while and you can verify it becomes **passing**. [You can also verify the log similar to this at Travis CI](https://travis-ci.org/steadylearner/travis-ci/builds).

```console
worker_info

Worker information

system_info

Build system information

docker_mtu
git.checkout

Setting up build cache

cache.yarn

cache.npm

$yarn --version

$eslint .

$jest

Ran all test suites.

The command "yarn test" exited with 0.

store build cache

Done. Your build exited with 0.
```

Find it tests instead of you with Docker containers and codes you left at **package.json**, .travis.yml etc.

<br />

## 6. Commit.py to automate the GitHub commit process

If you read the documentation of [Travis CI], you can find it is used with **GitHub** mostly. Make a file similar to **commit.py** with JavaScript, Python or others you want to commit it faster.

```py
import subprocess as cmd

cp = cmd.run("git add .", check=True, shell=True)

response = input("Do you want to use the default message for this commit?([y]/n)\n")
message = "update the repository"

if response.startswith('n'):
    message = input("What message you want?\n")

cp = cmd.run(f"git commit -m '{message}'", check=True, shell=True)
cp = cmd.run("git push -u origin master -f", check=True, shell=True)

```

Use **$python3 commit.py** in your console and you can commit your repository easily. Read [How to automatically commit files to GitHub with Python] for more information.

<br />

## 7. Conclusion

I hope you made it all work. We could set up Node Express development environment with Travis CI. In the process, we could also include Jest, Tape, Supertest, ESLint etc.

In the later [Steadylearner Blog], we will learn how to deploy the **web app** with [Elastic Beanstalk] from [AWS] and Dockerfile.

We will also learn how to deploy gRPC Node micro services with Postgresql database to [ECS], [Cloud​Formation], **docker-compose.yml** etc.

If you want the latest contents from Steadylearner, follow me at [Twitter].

Do you need **a Full Stack Developer** who can deploy the projects with Docker, AWS etc? Contact me with [LinkedIn] and I will help you.

**Thanks and please share this post with others**.
