<!--
  Post{
    subtitle:  "Some React components to help you write markdown with ease.",
    image:  "/code/React.png",
    image_decription: "React Image from the website",
    tags: "markdown React NPM react-easy-md",
  }
-->

<!-- Link -->

[react-marked-markdown]: https://github.com/Vincent-P/react-marked-markdown
[React Easy Markdown Github Repository]: https://github.com/steadylearner/react-easy-md
[examples]: https://github.com/steadylearner/react-easy-md/tree/master/examples
[Codesandbox for react-easy-md]: https://codesandbox.io/s/wz9pp1xpn8
[How to enable code syntax highlight in React App]: https://medium.com/@steadylearner/how-to-enable-code-syntax-highlight-in-react-app-38463498fa6e
[How to write less code for links in markdown with React]: https://www.steadylearner.com/blog/read/How-to-write-less-code-for-links-in-markdown-with-React
[marked]: https://github.com/markedjs/marked

<!-- / -->

<!-- Steadylearner -->

[Steadylearner]: https://www.steadylearner.com/
[Blog]: https://www.steadylearner.com/blog
[Markdown]: https://www.steadylearner.com/markdown
[prop-passer]: https://www.npmjs.com/package/prop-passer

<!-- / -->

<!-- [![Travis branch](https://img.shields.io/travis/Steadylearner/react-easy-md/master.svg?maxAge=2592000)]() -->

 [![npm version](https://badge.fury.io/js/react-easy-md.svg)](https://badge.fury.io/js/react-easy-md) [![npm](https://img.shields.io/npm/dt/react-easy-md.svg?maxAge=2592000)](https://img.shields.io/npm/dt/react-easy-md.svg)

# React Easy Markdown(react-easy-md)

It will help you to write Markdown with React easily.

---

<!-- It turns it to absolute path at NPM -->

[![react-easy-md-example](/static/images/post/react-easy-md/react-easy-md-example.png)](https://github.com/steadylearner/react-easy-md/tree/master/examples/react-easy-markdown-example)

The original code used here was from [react-marked-markdown][react-marked-markdown].

But the differences are

1. It solved the problem of showing `null` title.
2. `prefixAndReplacement` prop is included to help you write shortcuts for `<a>` inside markdown.
3. The modules used here became funtional components.
4. LiveMarkdownEditor is removed from the package to reduce package size and refer to [example code][examples] from [Markdown Editor Page][Markdown] at [Steadylearner][Steadylearner] instead if you want a working example.
5. You can use **API** such as **copy(ToClipBoard)**, **html** and **makrdown**.

To explain more about `1.`, You can define title in Markdown with a code such as
`[Website](https://www.steadylearner.com/ "Website")`.

But having default value solves the problem of showing **null** title when users forget to define it or when you get data from the other websites that don't have title value defined with `<a>` tag.

It will also be convenient to have default values to save your time and space in .md file.

The original Github repository is archived so this package was made to share the code from the former repository with some improvements.

The name of package became "react-easy-md" for the NPM Package didn't allow "react-easy-markdown" for similarity.

(You may think that React Easy Mardkdown refer to `react-easy-md` in this documentation.)

For [Steadylearner][Steadylearner] uses markdown intensively, it may include more features later.

## Install

1. Type `$npm install --save react-easy-md` or `$yarn add react-easy-md` in your **CLI**

2. Import component(s) you want

```js
import {
  MarkdownPreviw,
  MarkdownInput,
  // Below are functions you may need while you develop
  html,
  markdown,
  copy,
  readLocalFileWithHow, // How -> Function(How to use content of local file)
  saveTextFromWeb,
} from 'react-easy-md';
```

## Problem with Webpack?

If you see some warnings and errors with this package while you use webpack, you may include

```js
// Refer to webpack.config.js at
// https://github.com/steadylearner/react-easy-md/blob/master/examples/config/webpack.config.js

// Remove errors in developement
const FilterWarningsPlugin = require('webpack-filter-warnings-plugin');
// For production mode work, use it instead of uglifyjsplugin
const TerserPlugin = require('terser-webpack-plugin'); //

moudle.exports = () => {
  return({
    module: {

      rules: [
        {// to exclude source map error from third part libraires.
          test: /\.(js|jsx)$/,
          exclude: /node_modules/,
          use: ["source-map-loader"],
          enforce: "pre",
        },
      ],

      node: {
        fs: "empty",
        tls: "empty",
        net: "empty",
        child_process: "empty",
      },

      optimization: {
        minimizer: [new TerserPlugin()],
      },

      // To remove warning from 'jsdom' used inside react-easy-md
      plugins: [
        new FilterWarningsPlugin({
          exclude: /Critical dependency: the request of a dependency is an expression/,
        }),
      ],

    }
  })
}

// You should refer to TypeScript documentation
// if you had problem with it.
```

## Version Specific

1. **readLocalFileWithHow** to help you edit markdown directly from your local machine.
2. **saveTextFromWeb** to help you download your markdown draft with browser API.

## Example

You may read [How to enable code syntax highlight in React App] if you want to use code snippets inside your app or visit [react-marked-markdown][react-marked-markdown] for more information.

Every props used here is optional but it will be a starting point for your app. You can use **CSS** files in [examples folder][examples].

```js
// index.js
import React from "react";
import ReactDOM from "react-dom";
import { MarkdownPreview, copy, html, markdown } from "react-easy-md";

// Refer to www.steadylearner.com/markdown page
import example from "./example";
import "./styles.css";

// Include <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.13.1/styles/foundation.min.css">
// at index.html to test hljs work or not

function App() {
  const website = "https://www.steadylearner.com";
  return (
    <section className="App">
      <MarkdownPreview
        // For we allow html with santize false, it should show the same result
        // html and markdown API don't reserve html props well so use it with caution.
        value={markdown(html(example))}
        //
        markedOptions={{
          langPrefix: "hljs ", // hljs prefix to style code blocks.
          sanitize: false, // allow html
          breaks: true, // You can use [enter] to use \n
        }}
        // use absolute path to title attribute work well
        prefixWithReplacement={[
          ["s-", `${website}`],
          ["l-", "https://www.linkedin.com/in"],
          ["y-", "https://www.youtube.com/channel/"],
          ["t-", "https://twitter.com/"],
          ["compare-", `${website}/blog/read`],
          ["g-", "https://www.github.com"]
        ]} // You can define multiple shortcuts for links in markdown
      />
      <button onClick={() => copy(html(example))} >Copy</button>
      {/* <br /> */}
      <span className="blue"> and paste it to <a href="www.steadylearner.com/markdown">www.steadylearner.com/markdown</a></span>
      {/* <section>{html(example)}</section> */}
    </section>
  );
}

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);


```

## API

1. You can refer to [react-marked-markdown][react-marked-markdown] [marked][marked].
2. To understand **prefixWithReplacement** better, please visit [How to write less code for links in markdown with React][How to write less code for links in markdown with React].

### Usage of prefixWithReplacement

The part of the code snippet from the example above

```jsx
prefixWithReplacement={[
  ["s-", "https://www.steadlyearner.com"],
  ["l-", "https://www.linkedin.com/in"],
  ["y-", "https://www.youtube.com/channel/"],
  ["t-", "https://twitter.com/"],
  ["g-", "https://www.github.com"]
]}
```

We pass various **prefixes** with **its replacements** with data type **array of arrays**.

Then, Inside `MarkdownPreview` module it will convert

```md
[Blog](s-/blog)
[LinkedIn](l-/steady-learner-3151b7164)
[YouTube](y-/UCt_jsJOe91EVjd58kHpgTfw)
[Twittter](t-/steadylearner_p)
[Github](g-/steadylearner)

<!-- You can use it wherever you use link -->
<!-- [code]: s-/code "Steadylearner Code" -->
```

equal to

```md
[Blog](https://www.steadylearner.com/blog)
[LinkedIn](https://www.linkedin.com/in/steady-learner-3151b7164/)
[YouTube](https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw)
[Twittter](https://twitter.com/steadylearner_p)
[Github](https://github.com/steadylearner)
```

With `prefiexWithReplacement` from this package, **you don't have to type the entire paths anymore**.
It helps you **not to repeat what you know they will do**.

### html and markdown

```js
// This is just to show how it works
import { html, markdown } from "react-easy-md";

const package = "# react easy md";
const HTML = html(package); // <h1 id="react-easy-md" >react easy md</h1>
const react-easy-md = markdown(HTML) // "# react easy md"

```

### readLocalFileWithHow, saveTextFromWeb

```js
import React, { Component } from "react";
import { readLocalFileWithHow, saveTextFromWeb } from "react-easy-md";

class ReactEasyMarkdown extends Component {

  constructor(props) {
    super(props);
    this.state = {
      value: "",
    };
  }

  readLocalFile(e) {
      readLocalFileWithHow(e, (value) => this.setState({
          value,
      }));
  }

  render() {
    const { value } = this.state;
    return <span onClick={() => saveTextFromWeb(value)} >
  }
}
```

## Demo

1. [Steadylearner React-Easy-Markdown][Markdown]

2. [Steadylearner Blog Post for this package](https://www.steadylearner.com/blog/read/React-Easy-Markdown)

[![react-easy-md-example](/static/images/post/react-easy-md/react-easy-md-post.png)](https://www.steadylearner.com/blog/read/React-Easy-Markdown)

## What is Next?

1. **Tests**, **examples** and the webpage
2. [Posts][Blog] to explain it at [Steadylearner][Steadylearner]

## Read More

1. [Steadylearner Blog Posts for examples][blog]
2. [prop-passer to help you write less prop and className][prop-passer]

## Where to learn and use markdown?

 [Markdown-Tutorial]: https://www.markdowntutorial.com/

 1. [Start with Markdown-Tutorial][Markdown-Tutorial]
 2. [Markdown CheatSheet](https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet)
 3. [Use mark down for Github page](https://help.github.com/articles/getting-started-with-writing-and-formatting-on-github/)
 4. [Learn Markdown in X Minutes](https://learnxinyminutes.com/docs/markdown)
 5. [Steadylearner Markdown Live Editor][markdown]
 6. [Markdown to html](https://markdowntohtml.com/)
 7. [Markdown Interpreter](https://dillinger.io/)

