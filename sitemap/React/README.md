<!--
  Post{
    subtitle: "Learn how to make sitemap.xml with React Routes",
    image: "post/sitemap/sitemap-react.png,
    image_decription: "Made with CSS by Steadylearner",
    tags: "React How sitemap build",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Sitemap GitHub]: https://github.com/steadylearner/Sitemap
[What is image sitemap]: https://support.google.com/webmasters/answer/178636
[React Router Sitemap]: https://www.npmjs.com/package/react-router-sitemap

<!-- / -->

I built [Steadylearner]. Then, I thought that **"It is time to improve SEO with sitemap and metadata"**. 

It is made with **React** and **Rust**. So there were not enough examples yet. Therefore, it was not easy to find the sufficient information.

Furthermore, it was difficult to use server side rendering and other **Node** friendly methods from React for Rust. So I searched what can I do. Then, I found that sitemap and metadata can still be implemented only using **JavaScript**.

I want to share you how to do that with this post. We will use [React Router Sitemap] for the process.

Its goal is to extract only routes made with React Router and return **.xml** type sitemap.

I found that it was not complete to meet every requirements to make sitemap. But it will be still useful to start and find what you need.

Because you have to know how to deal with ES6+ JavaScript Codes, You may find that following [the documentation from the author][React Router Sitemap] is not easy.

So I will give you every files you need to make it work for your app.

I hope you already have [Node](https://nodejs.org/en/) installed and know JavaScript also before you follow this post.

(You may read the source code at [Sitemap GitHub] repository first. It will be sufficient for you to start your own project.)

We will start from installing the minimum JavaScript files.

You do not have to know a lot about how packages below work. We will use the project only to make React-Router-Sitemap work.

It can be used independently. So after reading this pots, You may use it for your own React-Router routes whenever you want.

The minimum **package.json** for the project would be

```json
{
  "name": "react-sitemap-builder",
  "version": "1.0.0",
  "main": "index.js",
  "author": "www.steadylearner.com",
  "license": "MIT",
  "devDependencies": {
    "@babel/core": "^7.3.4",
    "@babel/preset-env": "^7.3.4",
    "@babel/preset-react": "^7.0.0",
    "@babel/register": "^7.0.0",
    "babel-loader": "^8.0.5",
    "webpack": "^4.29.5"
  },
  "dependencies": {
    "react": "^16.8.3",
    "react-dom": "^16.8.3",
    "react-router": "^4.3.1",
    "react-router-sitemap": "^1.2.0",
    "webpack-cli": "^3.2.3",
    "webpack-dev-middleware": "^3.6.0",
    "webpack-dev-server": "^3.2.1"
  }
}
```

You can use **npm** or **yarn** to install the packages.

(For the sitemap will be only useful after you end up completing your website. You would't need detailed explanation for them.)

Then, the minimum webpack.config.js for the project would be

```js
const webpack = require("webpack");

module.exports = function () {
  return {
    module: {
      rules: [{
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: {
          loader: "babel-loader",
          options: {
            presets: ['@babel/preset-env']
          }
        }
      }]
    },
    devServer: {
      historyApiFallback: true,
    },
    resolve: {
      alias: {
        "react-dom": "react-dom/profiling",
        "scheduler/tracing": "scheduler/tracing-profiling"
      }
    }
  };
};
```

(I use webpack for this post for I use it for [Steadylearner].)

To make **babel-loader** and **@babel/preset-env** work is important here. They will help you to use **ES6+ syntax**. With it, you can follow the example of **React-Router-Sitemap**.

In accordance with the webpack file above, we will make **.babelrc**.

```json
{
    "presets": [
        "@babel/preset-env",
        "@babel/preset-react"
    ],
}
```

and you have the minimum **Webpack** and **Babel** files ready for the purpose of this post.

If some problem happens following this examples, you may test it with help of [babel-repl](https://babeljs.io/repl) to verify it work or not.

```js
// react-sitemap-test.js
"use strict";

var _react = _interopRequireDefault(require("react"));

var _reactRouter = require("react-router");

var _reactRouterSitemap = _interopRequireDefault(require("react-router-sitemap"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var sitemap = new _reactRouterSitemap.default(_react.default.createElement(_reactRouter.Route, {
  path: "/home"
})).build('http://www.steadylearner.com').save("./sitemap.xml");

console.log(sitemap);

// equals to

// import React from "react"
// import { Switch, Route, } from "react-router"
// import Sitemap from 'react-router-sitemap';

// const sitemap =
//  new Sitemap(<Route path='/home' />)
//    .build('http://www.steadylearner.com')
//    .save("./sitemap.xml")

// console.log(sitemap);
```

You can verify how it works with the **$node react-sitemap-test.js**. Then, you will see the messages like this in your console.

```xml
cache:'<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1">\n<url> <loc>http://www.steadylearner.com/home</loc> </url>\n</urlset>',
```

You should search information about [What is sitemap?](https://support.google.com/webmasters/answer/156184?hl=en), [How to build sitemap](https://www.google.com/search?client=firefox-b-d&q=how+to+build+sitemap) if you don't know what they mean.

Then, you will find that

```xml
<loc>http://www.steadylearner.com/home</loc>
```

is payload.

We can see that it is made from the code

```js
new Sitemap(<Route path='/home' />)
.build('http://www.steadylearner.com')
.save("./sitemap.xml")
```

We know that the minimum example works after the process before. We made a first sitemap with [React Router Sitemap]. What we should code is to use our own React Router routes instead.

Following the example from the documenation, we will make **sitemap-builder.js**

```js
require('@babel/register'); // 1.

const router = require('./router').default;
const Sitemap = require('react-router-sitemap').default;

(
    new Sitemap(router)
        .build('http://www.steadylearner.com')
        .save('./sitemap.xml')
); // 2.

console.log("The sitemap was built."); // Only shows this message after everything works well.
```

1. You should use **@prefix for babel/register** and the package official example used is not correct.

2. We do not have router variable to use with Sitemap(argument) yet, we will build it.

I corrected a code snippet to help you. The process to build sitemap.xml is almost done. We just need to make router variable. It will be passsed to construct Sitemap with syntax **new Sitemap(Router)**.

The code below is from the react router routes for [Steadylearner] to help you refer to the real example.

```js
import React from "react";
import { Switch, Route } from "react-router";

export default (
    <Switch>
        <Route exact path="/" />
        <Route exact path="/about" />
        <Route path="/about/:language?" />
        <Route exact path={"/video"} />
        <Route path={"/video/search/:query?"} />
        <Route path={"/video/watch/:videoId"} />
        <Route path={"/video/write/:videoId?"} />
        <Route exact path={"/image"} />
        <Route path={"/image/search/:query?"} />
        <Route exact path={"/blog"} />
        <Route path={"/blog/search/:query?"} />
        <Route path={"/blog/read/:blogTitle?"} />
        <Route exact path="/code" />
        <Route path={"/code/search/:query?"} />
        <Route exact path="/markdown" />
        <Route exact path="/jsx"/>
        <Route exact path={"/slideshow"} />
        <Route path={"/static/images/:folder"} />
    </Switch>
);
```

You may use your routes from React-Router. But I want you to remove unnecessary parts inside your routes.(Only the last paths the user visit will be used.)

So you should delete code such as **rediret** and components to make it more readable and not to make confusion for the React Router Sitemap to work.

Everything is ready.

Type **$node sitemap-builder.js**.

The console will show you the message **Sitemap was built** for you. Then, You can verify the result with **sitemap.xml**.

It will be similiar to

```xml
// sitemap.xml
<?xml version="1.0" encoding="UTF-8"?>

<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:news="http://www.google.com/schemas/sitemap-news/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml" xmlns:mobile="http://www.google.com/schemas/sitemap-mobile/1.0" xmlns:image="http://www.google.com/schemas/sitemap-image/1.1" xmlns:video="http://www.google.com/schemas/sitemap-video/1.1"
>

<url> <loc>http://www.steadylearner.com/</loc> </url>
<url> <loc>http://www.steadylearner.com/about</loc> </url>
<url> <loc>http://www.steadylearner.com/about/:language?</loc> </url>
<url> <loc>http://www.steadylearner.com/video</loc> </url>
<url> <loc>http://www.steadylearner.com/video/search/:query?</loc> </url>

</urlset>
```

If you like the result, you may submit it for some search engines.

I want you to learn how to do that by searching documentations yourself.

If you want to use a sitemap.txt file also. You can only use **path** in `<url><loc>$path</loc></url>` parts from it.

Then use * instead of dynamic path parts such as :language? and :query.

For example,

```txt
http://www.steadylearner.com/
http://www.steadylearner.com/about
http://www.steadylearner.com/about/*
http://www.steadylearner.com/video
http://www.steadylearner.com/video/search/*
http://www.steadylearner.com/video/watch/*
http://www.steadylearner.com/video/write/*
http://www.steadylearner.com/image
http://www.steadylearner.com/image/search/*
http://www.steadylearner.com/blog
http://www.steadylearner.com/blog/search/*
http://www.steadylearner.com/blog/read/*
http://www.steadylearner.com/code
http://www.steadylearner.com/code/search/*
http://www.steadylearner.com/markdown
http://www.steadylearner.com/jsx
http://www.steadylearner.com/slideshow
http://www.steadylearner.com/static/images/*
```

(You may use [this package](https://www.npmjs.com/package/react-router-sitemap-builder) instead to make sitemap.txt also.)

If you followed well this post, you already have boilerplate to start the sitemap for your React project.

I hope you made it and see your React project indexed by search engines.

Before the end of the post, there are some notes that I want to tell you.

Paths such as http://www.steadylearner.com/about/:language does not work well for search engines to index every paths in your React Website.

It may be better for you to write manually if there are a few paths.

For example, http://www.steadylearner.com/about/pt-br and http://www.steadylearner.com/about/es instead.

Otherwise, write dynamic sitemap with database and programming language for backend. You can make it more specific.

**Thanks and please share this post with others.**
