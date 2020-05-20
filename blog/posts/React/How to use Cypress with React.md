<!--
  Post {
    subtitle: "Set up Cypress and test it with your project.",
    image:  "/code/cypress.png",
    image_decription: "Image from the official website",
    tags: "How Cypress React use",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[React Official Website]: https://reactjs.org/
[How to use JavaScript]: https://developer.mozilla.org/en/docs/Web/JavaScript
[How to use NPM]: https://docs.npmjs.com/about-npm/
[Babel]: https://babeljs.io/
[How to use Babel]: https://babeljs.io/en/setup#installation
[Babel REPL]: https://babeljs.io/repl
[How to setup Jest for beginners]: https://jestjs.io/docs/en/getting-started.html
[How to setup Enzyme for beginners]: https://airbnb.io/enzyme/
[React Rust]: https://github.com/steadylearner/React-Rust
[React blog posts]: https://www.steadylearner.com/blog/search/React
[Rust blog posts]: https://www.steadylearner.com/blog/search/Rust
<!-- https://webpack.js.org/branding/ -->
[Webpack]: https://webpack.js.org/
[React]: https://reactjs.org/
[React Router]: https://reacttraining.com/react-router/web/guides/quick-start
[CRA]: https://github.com/facebook/create-react-app
[Yarn]: https://yarnpkg.com/lang/en/
[nvm]: https://github.com/nvm-sh/nvm
[node-sass]: https://github.com/sass/node-sass
[Formik]: https://github.com/jaredpalmer/formik
[Yup]: https://github.com/jquense/yup
[React Text Mask]: https://www.npmjs.com/package/react-text-mask
[Cypress]: https://www.cypress.io/
[Selenium]: https://www.seleniumhq.org/

<!-- / -->

<!-- Post -->

[How to turn class React component into functional component]: https://www.steadylearner.com/blog/read/How-to-turn-React-class-component-into-functional-component
[How to write less code for links in markdown with React]: https://www.steadylearner.com/blog/read/How-to-write-plugin-to-write-markdown-easily-with-React
[How to enable Code Syntax Highlight in React App]: https://medium.com/@steadylearner/how-to-enable-code-syntax-highlight-in-react-app-38463498fa6e?source=---------8------------------
[How to setup Jest to test JavaScript Code]: https://www.steadylearner.com/blog/read/How-to-setup-Jest-to-test-JavaScript-Code
[How to setup Jest with Enzyme to test React Code]: https://www.steadylearner.com/blog/read/How-to-setup-Jest-with-Enzyme-to-test-React-Code
[How to make es5 JavaScript code from es6+ with Babel]: https://www.steadylearner.com/blog/read/How-to-make-es5-JavaScript-code-from-es6+-with-Babel
[How to use Webpack with React]: https://www.steadylearner.com/blog/read/How-to-use-Webpack-with-React

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!-- / -->

<!-- Write code for Rust CLI to read the data from the github with conditional statements and remove this comment -->

In this post, we will learn how to set up [Cypress] project. Then, we will test it with the [React] project at [React Rust] GitHub repository.

Please, clone the repository first and follow the instruction there.

It will be similar to this.

[![user-signup](https://www.steadylearner.com/static/images//post/React/user-signup.png)][React Rust]

You should use **$yarn start:cypress** when you want to test it with **Cypress** later.

(The project used here is with Webpack split chunk API and I could find it is not so compatible with **Cypress** and made a separate commands at package.json and configurations at webpack.config.js.)

You can also use yours instead. Only modify **baseUrl** part used for webpack development server in **cypress.json**.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. JavaScript, NPM or [Yarn]
2. React
3. Selenium

---

You should know how to use **JavaScript** and **NPM** to download the packages used here from it.

I hope you are already familar with [React] because the example project is built with it. But, it is not requirements. Use site you want if what you want is just to learn Cypress.

[The project is built with Webpack to use React][How to use Webpack with React]. You can also use the project made with [CRA] or whatever project with other frameworks or website you want.

I am plan to write more [React blog posts] along with [Rust blog posts]. You can refer them later.

I thought Cypress is very similar to use [Selenium]. You can compare them also.

<br />

<h2 class="blue">Table of Contents</h2>

1. Install Cypress
2. Configure your project
3. Build your own test
4. How to record it
5. Conclusion

---

<br />

## 1. Install Cypress

We will first install Cypress with **package.json**. Make a file similar to this in whatever folder you want or inside your prexisting project.

I will use end_to_end_test for folder name with **$mkdir end_to_end_test**.

```json
{
  "name": "cypress_example_by_steadylearner",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT",
  "scripts": {
    "cypress": "cypress",
    "cypress:open": "cypress open",
    "cypress:all": "cypress run"
  },
  "devDependencies": {
    "cypress": "^3.6.0"
  }
}
```

Then, use **$yarn** or **$yarn add cypress --dev** to install **Cypress** as development dependency.

Type **$yarn cypress -v** or **$yarn cypress** in your console. It will show the result similar to this.

```console
  Usage: cypress <command> [options]

  Options:

    -v, --version      prints Cypress version
    -h, --help         output usage information

  Commands:

    help               Shows CLI help and exits
    version            prints Cypress version
    run [options]      Runs Cypress tests from the CLI without the GUI
    open [options]     Opens Cypress in the interactive GUI.
    install [options]  Installs the Cypress executable matching this package's version
    verify [options]   Verifies that Cypress is installed correctly and executable
    cache [options]    Manages the Cypress binary cache
```

Then, we verifed that it is installed well. So we will start **Cypress** with **$yarn cypress:open**. You can see that the command **Opens Cypress in the interactive GUI** in the message above.

It will show this explaning they made some helpful files and folder structures.

```console
We've added some folders and example tests to your project.
Try running the tests in the examples folder
or add your own test files to cypress/integration.
```

They are what we will learn exactly.

You can see that the folder structure is similar to this.

```console
end_to_end_test
  cypress
    fixtures
      example.json
    integration
      examples
        actions.spec.js
        aliasing.spec.js
    plugins
      index.js
    support
      commands.js
      index.js
```

There are many files but we don't need to learn how to use all of them. If you made it all work, click one of the tests by its authors.

[![user-signup](https://www.steadylearner.com/static/images//post/React/cypress-examples.png)][React Rust]

You can start with **local_storage.spec.js**. The main part of the [React Rust] project we will use for this post was **localStorage** API.

It will show the Cypress test in the separate window at [its localStorage relevant documenation page][https://example.cypress.io/commands/local-storage]. Test more examples and refer to the Cypress API used in there you like to use later.

```js
cy.get('.ls-btn').click().should(() => {
  expect(localStorage.getItem('prop1')).to.eq('red')
})
```

<br />

## 2. Configure your project

We already verified **Cypress** work in our machine with a few examples. Before we write our own tests, we will configure app to make it more usable for beginners.

We will start by editing **cypress.json**. It will be an empty object at this point.

```json
{
  "baseUrl": "http://localhost:8080",
  "video": false,
}
```

You won't want to save video files and slow your tests. It will be only useful when you become more familar with its API. So make it false at this point. We will learn how to use it in another part.

The baseUrl will be the location your server use or website. Use what you want.

Then, edit **cypress/plugins/index.js** with this. You can omit this process if you want to delete the examples.

```js
// ***********************************************************
// This example plugins/index.js can be used to load plugins
//
// You can change the location of this file or turn off loading
// the plugins file with the 'pluginsFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/plugins-guide
// ***********************************************************

// This function is called when a project is opened or re-opened (e.g. due to
// the project's config changing)

module.exports = (on, config) => {
  config.ignoreTestFiles = "**/examples/*.spec.js";
  return config;
};
```

For we just started using Cypress, it will be better to keep the examples. But, you won't want to show them in Cypress GUI everytime. This configuration will help you with that.

When you use Cypress again with **yarn cypress:open**. It will be similar to this without examples.

[![user-signup](https://www.steadylearner.com/static/images//post/React/cypress-without-examples.png)][React Rust]

<br />

## 3. Build your own test

The preparation ends and we can build our Cypress test file. We will use **usage.spec.js** built for [React Rust] as an example.

If you use a personal projet, you can refer to it fast and make your own file.

```js
describe("Test SignUp page at /", () => {
  it("Type all the correct datas at / and verify user and remove it at /user.", () => {
    // 1.
    cy.clearLocalStorage();

    // 2. CPF here is similar to Social Security Number
    const validName = "John Doe";
    const validEmail = "nome@exemplo.com";
    const validCpf = "111.111.111-11";
    const validTelefone = "88 98888-8888";

    const expectedUser = {
      nome: "John Doe",
      email: "nome@exemplo.com",
      cpf: "111.111.111-11",
      telefone: "(88) 98888-8888",
    };

    // 3.
    cy.visit("/");
    // mock submit with this if necessary .type("{enter}");;
    cy.get("input[id*=\"nome\"]").click().type(validName).should("have.value", validName);
    cy.get("input[id*=\"email\"]").click().type(validEmail).should("have.value", validEmail);
    cy.get("input[id*=\"cpf\"]").click().type(validCpf).should("have.value", validCpf);
    cy.get("input[id*=\"telefone\"]").click().type(validTelefone).should("have.value", "(88) 98888-8888");
    // 4. https://docs.cypress.io/api/commands/screenshot.html
    cy.screenshot();
    cy.get(".button--cadastrar").click().wait(2000);
    cy.screenshot();
    cy.get(".sign-up__main").trigger("keydown", { keyCode: 27, which: 27 });

    cy.get(".sign-up__main").should(() => {
      const users = JSON.parse(localStorage.getItem("users"));
      const { payload } = users;
      const user = payload[0];

      // Should use deep.equal for object comparision.
      // https://docs.cypress.io/guides/references/assertions.html
      expect(user).to.deep.equal(expectedUser);
      expect(payload.length).to.eq(1);
    });

    // 5.
    cy.visit("/user");
    cy.get(".user-list").click().wait(2000);
    cy.screenshot();
    cy.get(".user-list__remove").click().should(() => {
      const users = JSON.parse(localStorage.getItem("users"));
      const { payload } = users;
      const expectedUsers = [];

      expect(payload.length).to.eq(0);
      expect(payload).to.deep.equal(expectedUsers);

    });
  });
});
```

Nothing complicated here. If you used Jest or other frameworks, you will find it very similar to them. The differences will be thse.

1. It can't use **test** instead of **it**.
2. It is mostly used with browsers.

The purpose of the project is to save user data at localStorage with **SignUp** API and show that in **/user** page. Then, one can see the list of them and remove them there also.

The test file above is just to turn these behaviors to **Cypress JavaScript code**. 

The important parts are these.

1. First, we clear the **localStorage** to make it retestable everytime.
2. We prepare **input data** and **expected result**.
3. Visit the page and type data programmatically.
4. You will want to take **screenshot** at the processes you think important.
5. Visit another page. We need **wait** API for machines won't wait complete page to show up unlike humans.

You can test this after you make webpack development server ready with **$yarn start:cypress** in another console while your Cypress software is ready at the other console.

Click **usage.spec.js** in your Cypress window and it will show the test similar to this.

[![user-signup](https://www.steadylearner.com/static/images//post/React/cypress-test.png)][React Rust]

If you made it all work, you can verify that screenshots are saved at **/cypress/screenshots/usage.spec.js**.

```console
'Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user (1).png'
'Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user (2).png'
'Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user.png'
```

<br />

## 4. How to record it

Normally, you won't need to record your test. So we intentionally excluded video feature of Cypress before to test and learn it faster.

In this part, we will learn how to [record your tests and save them to video files with its API](https://docs.cypress.io/guides/guides/screenshots-and-videos.html).

You should sign up to Cypress first to use this. It won't be difficult if you are already GitHub user.

Click, **Runs** tab and its **Setup project to record**.

You will see the some default options. Use what you think better.

Then, you will see the result

```console
To record your first run...
1. Check cypress.json file into source control. Why?
{
  "projectId": "yours"
}
2. Run this command now, or in CI. Need help?
cypress run --record --key yours
```

When you read **cypress.json** file after this, you will see that it modifies it with **projectId** part you made.

```json
{
  "projectId": "yours"
}
```

You can run your test with the command given above.

```console
$cypress run --record --key yours
```

But, it is not easy to use those long words. Save it to **record-video.bash** and use whenever you want with **$./record-video.bash**.

Make it executable with **$sudo chmod +744 record-video.bash** first if you have a problem with it.

```bash
yarn cypress run --record --key yours
```

We do this approach because you won't want to save those commands with credentials in your **package.json** that should be used when you share your project.

If you use Ubuntu, you will need to install codec first. Use this or search more and find what you want yo use.

```console
$sudo apt install gstreamer1.0-libav
```

When everything is ready, use **./record-video.bash** while your project server is ready with **$yarn start:cypress**.

It will show the message simiar to this.

```console
 (Video)

  -  Started processing:  Compressing to 32 CRF
    Compression progress:  20%
    Compression progress:  45%
    Compression progress:  67%
    Compression progress:  91%
  -  Finished processing: /cypress/videos/   (43 seconds)
                          usage.spec.js.mp4


  (Uploading Results)

  - Done Uploading (1/4) /cypress/screenshots/usage.spec.js/Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user (2).png
  - Done Uploading (2/4) /cypress/screenshots/usage.spec.js/Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user (1).png
  - Done Uploading (3/4) /cypress/screenshots/usage.spec.js/Test SignUp page at -- Type all the correct datas at  and verify user and remove it at user.png
  - Done Uploading (4/4) /cypress/videos/usage.spec.js.mp4

====================================================================================================

  (Run Finished)


       Spec                                              Tests  Passing  Failing  Pending  Skipped
  ┌────────────────────────────────────────────────────────────────────────────────────────────────┐
  │ ✔  usage.spec.js                            00:45        1        1        -        -        - │
  └────────────────────────────────────────────────────────────────────────────────────────────────┘
    ✔  All specs passed!                        00:45        1        1        -        -        -


───────────────────────────────────────────────────────────────────────────────────────────────────────

  Recorded Run: https://dashboard.cypress.io/#/projects/yours/runs/1
```

You can visit **https://dashboard.cypress.io/#/projects/yours/runs/1** or find it at **cypress/videos/usage.spec.js.mp4**.

Normally, you won't need to record videos and it will make your machine to take more time to test your app. Use it only when it is necessary.

If you care for security and plan to share your project later, make a file such as .gitignore with this included.

```console
cypress.json
record-video.bash
```

<br />

## 5. Conclusion

I hope you made it work. We learnt how to setup and use **Cypress**. You could also share the screenshots and videos of your test.

In the later [React blog posts], we will learn how to use a [Formik], [Yup] and [React Text Mask] etc. It will help you learn how to make SignUp page used in this post to test Cypress API.

If you want the latest contents from Steadylearner, follow me at [Twitter] or star [React Rust].

Do you need **a Full Stack Developer**? Contact me with [LinkedIn] or [Twitter] and I will help you.

You can invite me to work with your team. **I can learn fast if there is a reason for that**.

**Thanks and please share this post with others.**
