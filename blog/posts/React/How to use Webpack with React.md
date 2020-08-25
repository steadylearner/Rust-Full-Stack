<!--
  Post {
    subtitle: "Set up React Project with Webpack and React Router.",
    image:  "/code/webpack.png",
    image_decription: "Image from the official website",
    tags: "How React Webpack code",
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
[React Rust]: https://github.com/steadylearner/React-Rust
[React Text Mask]: https://www.npmjs.com/package/react-text-mask

<!-- / -->

<!-- Post -->

[How to turn class React component into functional component]: https://www.steadylearner.com/blog/read/How-to-turn-React-class-component-into-functional-component

[How to write less code for links in markdown with React]: https://www.steadylearner.com/blog/read/How-to-write-plugin-to-write-markdown-easily-with-React

[How to enable Code Syntax Highlight in React App]: https://medium.com/@steadylearner/how-to-enable-code-syntax-highlight-in-react-app-38463498fa6e?source=---------8------------------

[How to setup Jest to test JavaScript Code]: https://www.steadylearner.com/blog/read/How-to-setup-Jest-to-test-JavaScript-Code

[How to setup Jest with Enzyme to test React Code]: https://www.steadylearner.com/blog/read/How-to-setup-Jest-with-Enzyme-to-test-React-Code

[How to make es5 JavaScript code from es6+ with Babel]: https://www.steadylearner.com/blog/read/How-to-make-es5-JavaScript-code-from-es6+-with-Babel

<!-- / -->

<!-- Steadylearner Twitter and LinkedIn  -->

[Twitter]: https://twitter.com/steadylearner_p
[LinkedIn]: https://www.linkedin.com/in/steady-learner-3151b7164/

<!-- / -->

<!-- Write code for Rust CLI to read the data from the github with conditional statements and remove this comment -->

In this post, we will learn how to use [Webpack] to build a [React] project with [React Router].

If you want a complete project first, please clone [React Rust] and follow the instruction there. The result will be similar to this.

[![user-signup](https://www.steadylearner.com/static/images//post/React/user-signup.png)][React Rust]

If you made it work, just take parts you will use from it.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. JavaScript, NPM or [Yarn]
2. React
3. How to make es5 JavaScript code from es6+ with Babel
4. [React Router] example

---

You should know how to use **JavaScript** and **NPM** to download the packages used here. I hope you are also already familar with [React].

I will use Yarn commands for this post. You can use equivalent NPM commands otherwise.

If you find the exmaple difficult and just want to have React code used for [the project][React Rust], you can also use [CRA].

The webpack configruation(webpack.cofig.js) of this post is very similar to what used at [Steadylearner].

I will use these [React blog posts] to update the website also. So **package.json** in [React Rust] will be updated along with them. Therefore, If you think the content of the post is outdated, please refer to [the repository][React Rust].

This is the following post of [How to make es5 JavaScript code from es6+ with Babel]. If you are not familar with it yet, refer to the post first.

Read [React Router] example documentation also.

<br />

<h2 class="blue">Table of Contents</h2>

1. Install dependencies
2. Set up **Webpack** with **Babel**
3. Start your project with **React Router**
4. Conclusion

---

<br />

## 1. Install dependencies

We will first install dependencies with **package.json**. It will be similar to this.

```json
{
  "name": "steadylearner_website",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT",
  "scripts": {
    "begin": "code ./ && webpack-dev-server --mode=development --open",
    "start": "webpack-dev-server --mode=development --open",
    "start:cypress": "webpack-dev-server --mode=development --env.cypress --open",
    "clean": "rimraf dist",
    "dev": "webpack --mode=development",
    "build": "yarn clean && webpack --mode=production --sourcemap=false",
    "static": "yarn build && git subtree push --prefix dist origin gh-pages",
    "fix": "eslint --ignore-path .gitignore . --fix",
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage",
    "test:memory": "jest --env=node --logHeapUsage",
    "test:u": "jest --updateSnapshot",
    "cypress": "cypress",
    "cypress:open": "cypress open",
    "cypress:all": "cypress run",
    "cypress:dev": "start-server-and-test start:cypress http://localhost:8080 cypress:all"
  },
  "devDependencies": {
    "@babel/core": "^7.1.6",
    "@babel/plugin-proposal-decorators": "^7.1.6",
    "@babel/plugin-proposal-do-expressions": "^7.0.0",
    "@babel/plugin-proposal-nullish-coalescing-operator": "^7.0.0-beta.54",
    "@babel/plugin-proposal-optional-chaining": "^7.0.0-beta.54",
    "@babel/plugin-proposal-pipeline-operator": "^7.0.0-beta.54",
    "@babel/plugin-syntax-dynamic-import": "^7.0.0",
    "@babel/plugin-transform-async-to-generator": "^7.0.0",
    "@babel/plugin-transform-runtime": "^7.0.0-beta.54",
    "@hint/hint-x-content-type-options": "^3.1.2",
    "@testing-library/react": "^9.3.1",
    "babel-core": "^7.0.0-bridge.0",
    "babel-eslint": "^10.0.1",
    "babel-jest": "^23.4.2",
    "babel-minify-webpack-plugin": "^0.3.1",
    "babel-plugin-styled-components": "^1.5.1",
    "cypress": "^3.6.0",
    "dynamic-cdn-webpack-plugin": "^4.0.0",
    "enzyme": "^3.3.0",
    "enzyme-adapter-react-16": "^1.1.1",
    "enzyme-to-json": "^3.3.4",
    "eslint": "^5.4.0",
    "eslint-config-airbnb": "^17.1.0",
    "eslint-plugin-import": "^2.14.0",
    "eslint-plugin-jsx-a11y": "^6.1.1",
    "eslint-plugin-node": "^8.0.0",
    "eslint-plugin-react": "^7.11.1",
    "eslint-plugin-react-hooks": "^1.4.0",
    "jest": "^24.9.0",
    "module-to-cdn": "^3.1.2",
    "regenerator-runtime": "^0.13.1",
    "source-map-loader": "^0.2.3",
    "start-server-and-test": "^1.10.6",
    "terser-webpack-plugin": "^1.2.3",
    "webpack-bundle-analyzer": "^3.0.3",
    "webpack-filter-warnings-plugin": "^1.2.1"
  },
  "dependencies": {
    "@babel/plugin-proposal-class-properties": "^7.0.0",
    "@babel/plugin-proposal-do-expressions": "^7.0.0",
    "@babel/plugin-proposal-object-rest-spread": "^7.0.0",
    "@babel/preset-env": "^7.0.0",
    "@babel/preset-react": "^7.0.0",
    "@babel/runtime": "^7.0.0-beta.49",
    "autoprefixer": "^9.1.2",
    "babel-loader": "^8.0.0",
    "babel-plugin-syntax-dynamic-import": "^6.18.0",
    "clean-webpack-plugin": "^1.0.0",
    "connect-multiparty": "^2.1.1",
    "csp-webpack-plugin": "^2.0.2",
    "css-loader": "^2.1.0",
    "file-loader": "^3.0.1",
    "formik": "^1.0.2",
    "html-loader": "^0.5.5",
    "html-webpack-plugin": "^3.2.0",
    "mini-css-extract-plugin": "^0.5.0",
    "node-sass": "^4.12.0",
    "postcss-loader": "^3.0.0",
    "prop-passer": "^1.1.1-3",
    "prop-types": "^15.7.2",
    "react": "^16.11.0",
    "react-dom": "^16.11.0",
    "react-hotkeys-hook": "^1.5.3",
    "react-router-dom": "^4.3.1",
    "react-spring": "^8.0.27",
    "react-text-mask": "^5.4.3",
    "recompose": "^0.30.0",
    "sass-loader": "^7.0.1",
    "style-loader": "^0.23.1",
    "styled-components": "^4.1.2",
    "url-loader": "^1.1.1",
    "webpack": "^4.29.6",
    "webpack-cli": "^3.0.8",
    "webpack-dev-middleware": "^3.1.3",
    "webpack-dev-server": "^3.1.3",
    "yup": "^0.26.6"
  },
  "engines": {
    "node": "12.3.1"
  },
  "jest": {
    "verbose": true,
    "automock": true,
    "setupTestFrameworkScriptFile": "<rootDir>/src/__tests__/setup/setupEnzyme.js",
    "testPathIgnorePatterns": [
      "<rootDir>/src/__tests__/setup/"
    ]
  }
}
```

It has many dependencies but we will learn how to use most of them later with more [React blog posts]. If you find it complicated, refer to a whole project at [React Rust] and experiment it with the instruction there.

You can see that **node 12.3.1** is used here. This is to be compatible with **node-sass** verison of this package.json file.

You may update the package and Node with [nvm] and test it. Otherwise, rename index.scss in project and remove the package. It is just to show scss work.

<br />

## 2. Set up Webpack with Babel

We already installed the dependencies to use Webpack. In the previous part, we also needed to install Babel packages to help Webpack make es6+ JavaScript code with them.

We will make **.babelrc** first. It will be similar to this and you can delete what you won't use later.

```json
{
    "presets": [
        "@babel/preset-env",
        "@babel/preset-react"
    ],
    "plugins": [
        "styled-components",
        "@babel/plugin-proposal-class-properties",
        "@babel/plugin-proposal-object-rest-spread",
        "@babel/plugin-transform-async-to-generator",
        "@babel/plugin-transform-runtime",
        "@babel/plugin-proposal-do-expressions",
        "@babel/plugin-proposal-optional-chaining",
        "@babel/plugin-proposal-nullish-coalescing-operator",
        "@babel/plugin-syntax-dynamic-import",
        [
            "@babel/plugin-proposal-decorators",
            {
                "legacy": true
            }
        ],
        [
            "@babel/plugin-proposal-pipeline-operator",
            {
                "proposal": "minimal"
            }
        ]
    ],
    "env": {
        "test": {
            "plugins": [
                "@babel/plugin-transform-modules-commonjs"
            ]
        }
    }
}
```

Then, we are ready to use our **webpack.config.js**. It will be similar to the code snippet below.

**Finding the working Webpack configuration is not easy**. So verify the project compile first and read the documentations for each of them thoroughly.

```js
const webpack = require("webpack");

const HtmlWebPackPlugin = require("html-webpack-plugin"); // https://github.com/jantimon/html-webpack-plugin
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");
const CSPWebpackPlugin = require("csp-webpack-plugin"); // allow you use CDN when there is security problem
const autoprefixer = require("autoprefixer");

const TerserPlugin = require("terser-webpack-plugin"); // Use it instead of uglify https://stackoverflow.com/questions/47439067/uglifyjs-throws-unexpected-token-keyword-const-with-node-modules

// https://webpack.js.org/guides/environment-variables/
module.exports = (e) => {
  // console.log(e);
  // console.log("PAYLOAD", e.cypress);

  const minimizer = [
    new TerserPlugin(),
  ];

  let optimization = {
    minimizer,
    splitChunks: {
      cacheGroups: { // separate vendors
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: "vendors",
          chunks: "all",
        },
      },
    },
  };

  if (e !== undefined && e.cypress === true) {
    optimization = { minimizer };
  }

  return {
    module: {

      // https://webpack.js.org/configuration/module/#condition exclude unecessary files
      rules: [
        {
          test: /\.(js|jsx)$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader",
          },
        },
        {
          test: /\.(js)$/,
          exclude: /node_modules/,
        },
        { // to remove source map error from 3rd libraires such as rxjs from the console.
          test: /\.(js|jsx)$/,
          use: ["source-map-loader"],
          enforce: "pre",
        },
        {
          test: /\.html$/,
          use: [
            {
              loader: "html-loader",
              options: { minimize: true },
            },
          ],
        },
        {
          test: /\.(scss|css)$/,
          use: [
            MiniCssExtractPlugin.loader, // This plugin should be used only on production builds without style-loader
            {
              loader: "css-loader",
            },
            {
              loader: "postcss-loader",
              options: {
                autoprefixer: {
                  browsers: ["last 2 versions"],
                },
                plugins: () => [
                  autoprefixer,
                ],
              },
            },
            {
              loader: "sass-loader",
              options: {},
            },
          ],

          // It works with CSS, Sass, CSS Module, React Specific(Inline Style and styled components)
        },

        {
          test: /\.(png|jp(e*)g|svg)$/,
          use: [{
            loader: "url-loader",
            options: {
              limit: 50000, // Convert images less than 50kb
              // limit: 8000, // Convert images less than 8kb
              name: "images/[hash]-[name].[ext]",
            },
          }],
        },
      ],
    },
    // It is important to include it to use react-easy-md without errors
    node: {
      fs: "empty",
      tls: "empty",
      net: "empty",
      child_process: "empty",
    },

    // historyApiFallback to handle browser unsync problem with webpack and react router
    // proxy to handle api CORS problem(https://webpack.js.org/configuration/dev-server/#devserver-proxy)

    devServer: {
      historyApiFallback: true,
      proxy: {
        "/api": "http://localhost:8000",
        "/static": "http://localhost:8000",
      },
    },

    // url-loader -> Images, fonts etc

    optimization,

    resolve: { // https://gist.github.com/bvaughn/25e6233aeb1b4f0cdb8d8366e54a3977
      alias: {
        "react-dom": "react-dom/profiling",
        "scheduler/tracing": "scheduler/tracing-profiling",
      },
    },

    plugins: [
      new webpack.ProgressPlugin(), // Show progress
      new webpack.optimize.AggressiveMergingPlugin(),
      new HtmlWebPackPlugin({ // https://github.com/jantimon/html-webpack-plugin
        template: "./src/index.html",
        filename: "./index.html",
      }),
      new CSPWebpackPlugin({
        "object-src": "\"none\"",
        "base-uri": "\"self\"",
        "script-src": [
          "\"unsafe-inline\"",
          "\"self\"",
          "\"unsafe-eval\"",
          "https://cdnjs.cloudflare.com/ajax/libs/babel-standalone/6.18.1/babel.min.js",
          "https://apis.google.com/",
        ],
        "worker-src": ["\"self\"", "blob:"],
      }),
      new MiniCssExtractPlugin({
        filename: "[name].css",
        chunkFilename: "[id].css",
      }),
      new CleanWebpackPlugin(["public"]), // It inscreases process speed so much.

    ],
  }
};
```

With this configuration, you can use React, SCSS, images and almost all the things available in [Steadylearner].

[We will learn how to use Cypress](https://www.steadylearner.com/blog/read/How-to-use-Cypress-with-React) later. We need the code snippet similar to this for that.

```js
const minimizer = [
  new TerserPlugin(),
];

let optimization = {
  minimizer,
  splitChunks: {
    cacheGroups: { // separate vendors
      commons: {
        test: /[\\/]node_modules[\\/]/,
        name: "vendors",
        chunks: "all",
      },
    },
  },
};

if (e !== undefined && e.cypress === true) {
  optimization = { minimizer };
}
```

It will help you to use commands **start** and **start:cypress** in **package.json** separately. We need this because Cypress doesn't work well with split JavaScript files made from **splitChunks** part above.

You can remove it if you don't want to use Cypress.

<br />

## 3. Start your project with React Router

We prepared all the configuration files. If you have already visited [React Rust] repository. you will find that it has many files.

We won't handle every details of them. Normally, what you want would be Webpack and its relevant configuration files.

Build a **src** folder to include the files for our React project. The main folders will be similar to this.

```console
├── App.js
├── components
├── images
├── index.html
├── index.js
├── main.scss
```

You can use your CSS framework files here instead of **main.scss**.

In **index.html**, its payload will be this. It is the entry point of your app.

```html
<body>
  <div id="app"></div>
</body>
```

It is used to find it inside App.js

```jsx
import React, { StrictMode } from "react";
import ReactDOM from "react-dom";
import App from "./components";

const app = document.getElementById("app");

ReactDOM.render(
	<StrictMode><App /></StrictMode>,
	app,
);

export default App;
```

You could also include image files in **images** folder. Then, you can use path like **/src/images/image.jpg** to include images in your React project.

When you set up all of this, we can finally write the React codes relevant to your project. Move to the **components** folder.

If you already cloned [React Rust] repository, there will be **index.js**, **Main.js** and others. The structure was imporved from the code at [React Rotuer] example. Compare it with the **index.js**, **Main.js**, **TopNav/** used in this project.

They are just more modulized version of it.

**index.js** will be similar to this. You could also test it work with a single monolithic file from [React Rotuer] or whatever you can find in the internet first.

```js
import React from "react";
import {
	BrowserRouter as Router,
} from "react-router-dom";

import { TopNav } from "./UI";
import Main from "./Main";

import CustomCSS from "./CSS";

export default function Root() {
	return (
		<Router>
			<CustomCSS>
				<TopNav />
				<Main />
			</CustomCSS >
		</Router>
	);
}
```

It will be used to represent the total layout of your React Project.

Then, most of your React code should be in **Main.js**.

```js
import React, { Suspense } from "react";
import {
	Switch,
	Route,
} from "react-router-dom";

import Home from "./Home";
import User from "./User";

import Loader from "./Reusable/Loader";

export default function Main() {
	return (
		<Suspense
			fallback={
				<Loader>
					<i className="circle-spinner--s fixed border-round" />
					<i
						className="
							fab fa-youtube
							youtube-spinner--s
							fixed
							red
						"
					/>
				</Loader>
			}
		>
			<Switch>
				<Route exact path="/">
					<Home />
				</Route>
				<Route path="/user">
					<User />
				</Route>
			</Switch>
		</Suspense>
	);
}
```

Use **$yarn start** if you haven't yet at this point and verify it compile or not.

If you could make it work, you can start your React project by modifying **path** parts and including new React modules instead of **Home** and **User**.

<br />

## 3. Conclusion

I hope you made it work. It has been a long time after I made these configuration files. So I wanted to make a simple post to explain the workflow to make it work.

In the later [React blog posts], we will learn how to use a [Formik], [Yup] and [React Text Mask] etc. You will learn how to make a decent React form example with them.

If you want the latest contents from Steadylearner, follow me at [Twitter] or star [React Rust].

Do you need **a Full Stack Developer**? Contact me with [LinkedIn] or [Twitter] and I will help you.

You can invite me to work with your team. I can learn fast if there is a reason for that.

**Thanks and please share this post with others**.

