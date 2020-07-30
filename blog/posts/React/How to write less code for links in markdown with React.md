<!--
  Post {
    subtitle:  "Learn how to write less markdown with JavaScript Plugin.",
    image:  "code/markdown-preview-result-with-code.png",
    image_decription: "Result code from React Markdown Preview",
    tags: "React JavaScript code",
  }
-->

<!-- Link  -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[React Official Website]: https://reactjs.org/
[React Hook API]: https://reactjs.org/docs/hooks-intro.html
[React Spring]: https://react-spring.surge.sh/
[Github Repository for React Marked Markdown]: https://github.com/vincent-p/react-marked-markdown#readme
[React Markdown Improved]: https://codesandbox.io/s/wz9pp1xpn8
[How to use markdown]: https://www.markdowntutorial.com/

<!-- / -->

<!-- Post -->

[How to turn class React component into functional component]: https://www.steadylearner.com/blog/read/How-to-turn-React-class-component-into-functional-component

[How to write less code for links in markdown with React]: https://www.steadylearner.com/blog/read/How-to-write-plugin-to-write-markdown-easily-with-React

[How to enable Code Syntax Highlight in React App]: https://medium.com/@steadylearner/how-to-enable-code-syntax-highlight-in-react-app-38463498fa6e?source=---------8------------------

<!-- / -->

For blog posts for [Steadylearner][Steadylearner] uses markdown intensively, I have to deal with it many times. So I wanted to write some posts for it and find the better way to write it.

Before I begin to use markdown for the site with React Frontend, I had to find markdown renderer for it, There were some options to render markdown with **React** and I end up using [React Marked Markdown][Github Repository for React Marked Markdown].

It worked well and its **API** is also easy to use. It  also helps you to use [code syntax highlight work with it][How to enable Code Syntax Highlight in React App] to write posts about programming also. 

But it was with a problem. It shows a `null` for title when `<a>` tag is used to link other pages. So I thought that it will be better to give default value for title in `<a>` when the package renders markdown not to show the **null** value. 

It wasn't so difficult but the package is archived and couldn't find how to contribute to source code. I had to modify a single line to solve the problem explained before everytime want to prepare **Frontend** part for [the website][Steadylearner].

By writing this post and other posts relevant to it later, We needn't to repeat that anymore and I also paln to help others by making **NPM** modules that includes markdown renderer explained here later.

You may visit [Steadylearner Github Repository][Steadylearner Github Repository] to find source code or use steadylearner NPM module later if you want. Otherwise, reading this post will be sufficient for you to write your own markdown renderer with React. 

The final goal of this post is to help you write 

```md
[Blog](s-/blog)
[LinkedIn](l-/steady-learner-3151b7164)
[YouTube](y-/UCt_jsJOe91EVjd58kHpgTfw)
[Twittter](t-/steadylearner_p)
[Github](g-/steadylearner)
``` 
instead of 
```md
[Blog](https://www.steadylearner.com/blog)
[LinkedIn](https://www.linkedin.com/in/steady-learner-3151b7164/)
[YouTube](https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw)
[Twittter](https://twitter.com/steadylearner_p)
[Github](https://github.com/steadylearner)
```
when you write links inside the markdown. 

You can skip **1. How to solve null title problem for `<a>` tag in React-Marked-Markdown** if you just want to learn how to do that.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use markdown][How to use markdown]
2. [How to use React][React Official Website]
3. [React Marked Markdown][Github Repository for React Marked Markdown]
4. [Codesandbox Example for this post][React Markdown Improved]

---

You should know a little bit about how **markdown** and **React** work first to follow this post. Then, you may read the source code from the [Github Repository for React Marked Markdown][Github Repository for React Marked Markdown]. It will help you to understand this post better. You can also visit [Codesandbox Example for this post][React Markdown Improved] to play with the code used here. 

<br />

<h2 class="blue">Table of Contents</h2>

1. How to solve `null` title problem for `<a>` tag in **React-Marked-Markdown**
2. How to improve MarkdownPreview to write less code for links
3. How to make it work for multiple prefixes
4. **Conclusion**

---

<br />

## 1. How to solve null title problem for `<a>` tag in React-Marked-Markdown

I will show you the code snippet that is slightly different from the [Github Repository for React Marked Markdown][Github Repository for React Marked Markdown] first. 


```jsx
import React from 'react';
import marked from 'marked';
import hljs from 'highlight.js';
import PropTypes from 'prop-types';
import createDOMPurify from 'dompurify';
import { JSDOM } from 'jsdom';

// Initializing DOM Purify
const window = (new JSDOM('')).window;
const DOMPurify = createDOMPurify(window);

export default class MarkdownPreview extends React.Component {
  constructor(props) {
    super(props);

    let options = {};
    if (this.props.markedOptions) {
      options = this.props.markedOptions;
    }

    options = {
      gfm: true,
      tables: true,
      breaks: false,
      pedantic: false,
      sanitize: true,
      smartLists: true,
      smartypants: false,
      langPrefix: 'hljs ',
      ...options,
    };

    if (typeof hljs !== 'undefined') {
      options = {
        ...options,
        highlight: (code, lang) => {
          if (!!(lang && hljs.getLanguage(lang))) {
            return hljs.highlight(lang, code).value;
          }

          return code;
        },
      };
    }

    marked.setOptions(options);
  }
  render() {
    const { value, className } = this.props;
    const renderer = new marked.Renderer();
    renderer.link = (href, title, text) => (
      `<a target="_blank" rel="noopener noreferrer" href="${href}" title="${title === null ? `External link to ${href}` : title}">${text}</a>`
    );

    const html = DOMPurify.sanitize(marked(value || '', { renderer }));

    return (
      <div
        dangerouslySetInnerHTML={{ __html: html }}
        className={className}
      />
    );
  }
}

MarkdownPreview.propTypes = {
  value: PropTypes.string.isRequired,
  className: PropTypes.string,
  markedOptions: PropTypes.object,
};

MarkdownPreview.defaultProps = {
  value: '',
};

```
In the code snippet above, we can first assume that the author of the crate included the part below to intentionally include 

1. `target="_blank"` 
2. `rel="noopener noreferrer"` 

for `<a>` html tag  and not to make problem with React.
(You may have seen many warnings that you should include  both of them for `<a>` when you develop application with **React**.)

```jsx
const renderer = new marked.Renderer();
renderer.link = (href, title, text) => (
  `<a target="_blank" rel="noopener noreferrer" href="${href}" title="${title === null ? `External link to ${href}` : title}">${text}</a>`
);

const html = DOMPurify.sanitize(marked(value || '', { renderer }))
```

But in the official source code, there was no code to handle when title value is **null** so it showed `null` for each `<a>` tag rendered by the package. So what we had to do was to write code to handle that case. 

For value for `title` attribute is more for personal choice,  you can use your preference and the code below ẁas used as an example. 

```js
title="${title === null ? `External link to ${href}` : title}"
```

So we could solve the problem of showing `null` title. But it wouldn't be sufficient to write a blog post and later to include it for another NPM package. 

**Let's improve the example in the next phase.**

(I didn't explainted about how the entire source code works. If you want to understand it better you can always refer to the official documentations.)

<br />

## 2. How to improve MarkdownPreview to write less code for links

We already solved the main problem in the previous part. But the packagage is archived. It wouldn't worth trying to use the corrected code while rewriting a single line everytime you upgrade your package and the code is reverted to the previous one.

So what we gonna do is to improve it a little bit more. Let's start with making the **MarkdownPreview** module to functional component. 

(It was **React** class component without any state or method. We can make it more easy to use by turning it into functional component. You may refer to [How to turn class React component into functional component][How to turn class React component into functional component] to understand the process.)

The entire code will be like the code snippet below.

```jsx
// MarkdownPreview.js written with function instead of class
import React from "react";
import marked from "marked";
import hljs from "highlight.js";
import PropTypes from "prop-types";
import createDOMPurify from "dompurify";
import { JSDOM } from "jsdom";

// Initializing DOM Purify
const window = new JSDOM("").window;
const DOMPurify = createDOMPurify(window);

function MarkdownPreview({
  value,
  className,
  markedOptions,
  // 1.
  prefixWithReplacement,
  titleMessage
}) {
  let options = {};
  if (markedOptions) {
    options = markedOptions;
  }

  options = {
    gfm: true,
    tables: true,
    breaks: false,
    pedantic: false,
    sanitize: true,
    smartLists: true,
    smartypants: false,
    langPrefix: "hljs ",
    ...options
  };

  if (typeof hljs !== "undefined") {
    options = {
      ...options,
      highlight: (code, lang) => {
        if (!!(lang && hljs.getLanguage(lang))) {
          return hljs.highlight(lang, code).value;
        }

        return code;
      }
    };
  }

  marked.setOptions(options);

  const renderer = new marked.Renderer();
  // 2.
  renderer.link = (href, title, text) => {
    const hrefReplacedByPrefix = href.startsWith(prefixWithReplacement[0])
      ? `${prefixWithReplacement[1]}${href.split(prefixWithReplacement[0])[1]}`
      : href;
    return `<a target="_blank" rel="noopener noreferrer" href="${hrefReplacedByPrefix}" title="${
      title === null ? `${titleMessage} ${hrefReplacedByPrefix}` : title
    }" >${text}</a>`;
  };

  const html = DOMPurify.sanitize(marked(value || "", { renderer }));

  return (
    <div dangerouslySetInnerHTML={{ __html: html }} className={className} />
  );
}

MarkdownPreview.propTypes = {
  value: PropTypes.string,
  className: PropTypes.string,
  markedOptions: PropTypes.object,
  prefixWithReplacement: PropTypes.arrayOf.string,
  titleMessage: PropTypes.string
};
// or use other default value
MarkdownPreview.defaultProps = {
  value: "**This is default value. Write Your own markdown**",
  // 3.
  prefixWithReplacement: ["s-", "https://"],
  titleMessage: "Click it will open a new tab at"
};

export default MarkdownPreview;

```
If you read the code, you should have noticed that there were some modification from the previous **React class components**. 

If you have used links with markdown many times , What you should do were just to copy and paste. But if you had shortcut for them, It will be more easier to use them. 

For example, If you are the owner of the site, You shouldn't want to type 
`[Steadlyeanrer Website](https://www.steadylearner.com`)
`[Steadylearner Blog](https://www.steadylearner.com/blog)`
`[Steadlyeanrer Code](https://www.steadylearner.com/code)`
`[Steadlyeanrer Image](https://www.steadylearner.com/image)`
all the url for links similar to above. For you already know what you are doing and `https://www.steadylearner.com/` repeating itself.

and also you may have found that you almost alway use links from the same sites such as **Github**, **Twitter**, **YouTube** etc. With the power of programming, we don't have to type all of the links anymore inside markdown. 

We already talked about `a`tag in the previous part and we will modify the code used in the previous part.From now on, let's imporve it to help you to save time with few lines of code. You can refer to the numbers to understand what main code for that purpose does.

1. We will write more destuctured arguments for **MarkdownPreview** functional component such as **linkPrefix** and **titleMessage**. You can use them to personalize the component later.
2. You may have read that **prefixWithReplacement** is expected to be array in the code above. We will use **the first member of it(prefixWithReplacement[0])** as a prefix to be replaced by **the other member of it(prefixWithReplacement[1])**. It will only happen when the value passed to **href** attribute is started with **linkPrefix[0]**. We explain that intention to the machine with
```js
const hrefReplacedByPrefix = href.startsWith(prefixWithReplacement[0])
    ? `${prefixWithReplacement[1]}${href.split(prefixWithReplacement[0])[1]}`
    : href;
```
and pass the replaced href instead of the original href to render `<a>` tag.
3. We define default value for arguments for MarkdownPreview here. **You can use whatever you want for them**. Just refer to the ones used here for your project.

(In `2.` Array data format is intentionallyused to group prefix and its replacement. We will use that to define multiple prefix for links later.)

You can finally test it works with code snippet below. 

```jsx
// index.js, 
import React from "react";
import ReactDOM from "react-dom";

import test from "./test"; // Visit https://codesandbox.io/s/wz9pp1xpn8 
import MarkdownPreview from "./MarkdownPreview";
import "./styles.css";

function App() {
  return (
    <div className="App">
      <MarkdownPreview
        // Comment it to show default value or [Steadylearner Blog](s-/blog)
        // to test it equals to [Steadylearner Blog](https://www.steadlyearner.com/blog)
        value={test} 
        markedOptions={{
          langPrefix: "hljs ", // hljs prefix hljs relevant classes for styling
          sanitize: false, // allow html
          breaks: true
        }}
        prefixWithReplacement={["s-", "https://www.steadlyearner.com"]} // [prefix, replacement]
      />
    </div>
  );
}

const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);
```

and that was all you need to know to understand how to use shorcut such as 

```md
[Steadylearner Blog](s-/blog)
``` 
instead of 
```md
[Steadylearner Blog](https://www.steadylearner.com/blog)
```
 for markdown links inside your **.md** file with React.

It would be sufficient for the most of the users. But it will be better we can define multiple prefixes and their replacemnets for links.

In the next phase, we will rewrite the code snippets before to make them work for the purpose.

<br />

## 3. How to make it work for multiple prefixes
We will use FP  for this process. We used array data type for **prefixWithReplacment** before. It is very useful to apply the concept of **Functional Programming** and it will help you to write less code.

Let me show you the result code first

```jsx
// inside MarkdownPreview.js, replace the only parts described here

<Code snippet>

  renderer.link = (href, title, text) => {
    // 1.
    const isHrefeIncludeAnyPrefix = prefixWithReplacement.filter(x => href.startsWith(x[0]));
    if(isHrefeIncludeAnyPrefix.length > 0) { // 2.
      // 3.
      const hrefReplacedByPrefix = `${isHrefeIncludeAnyPrefix[0][1]}${href.split(isHrefeIncludeAnyPrefix[0][0])[1]}` 
      return `<a target="_blank" rel="noopener noreferrer" href="${hrefReplacedByPrefix}" title="${
        title === null ? `${titleMessage} ${hrefReplacedByPrefix}` : title
      }" >${text}</a>`;
    } else {
      return `<a target="_blank" rel="noopener noreferrer" href="${href}" title="${
        title === null ? `${titleMessage} ${href}` : title
      }" >${text}</a>`;
    }
  };

<Code snippet>

MarkdownPreview.propTypes = {
  value: PropTypes.string,
  className: PropTypes.string,
  markedOptions: PropTypes.object,
  prefixWithReplacement: PropTypes.arrayOf.arrays, // 4.
  titleMessage: PropTypes.string
};
// or use other default value
MarkdownPreview.defaultProps = {
  value: "**This is default value. Write Your own markdown**",
  prefixWithReplacement: [["s-", "https://"]], // 5.
  titleMessage: "Click it will open a new tab at"
};

```

```jsx
// index.js

<code snippet>

function App() {
  return (
    <div className="App">
      <MarkdownPreview
        value={test} // Comment it to show default value
        markedOptions={{
          langPrefix: "hljs ", // hljs prefix hljs relevant classes for styling
          sanitize: false, // allow html
          breaks: true
        }}
        prefixWithReplacement={[
          ["s-", "https://www.steadlyearner.com"],
          ["l-", "https://www.linkedin.com/in"],
          ["y-", "https://www.youtube.com/channel/"],
          ["t-", "https://twitter.com/"],
          ["g-", "https://www.github.com"],
        ]} // it can be plural(array of arrays)
      />
    </div>
  );
}

// Compare it to the code snippet before it was just a single array
// Its name is equal not to make it any longer and it can also be a single array.
// prefixWithReplacement={["s-", "https://www.steadlyearner.com"]}

<code snippet>
```

Understanding this code may be not so easy at first glance. You can just use its benefit and skip this part if you want. 

1. We use **filter** from **FP** to verfiy that **href** has any **prefix** that passed to **prefixWithReplacement**. You can use for loop or whatever you want but using it help you to solve all processes that we need with a single line.
2. The length of the **isHrefeIncludeAnyPrefix** can only be 1 or 0. So we use this conditional statement to simplify the process. If you think that you need more control. You can write your own code.
3. We used data type **array of arrays** for **prefixWithReplacement*.  Inside this code, it can only be array of array with a single member.** Therefore, we replace **prefixWithReplacement **with the **isHrefeIncludeAnyPrefix** from the code snippet in the previous part and **include `[0]`** for each part to select the parts that does the same role like the code snippet before. 
4. For type of data for prefixWithReplacement is diffferent, we use **PropTypes.arrayOf.arrays** instead.
5. It should be array of arrays to previous code work so we make it to [["s-", "https://"].

and that was all to write less code for links in markdown with React. The above code is used for prefixes but you can use it to make shorcut for entire path and it can be applied for other usages also.

<br />

## 4. Conclusion

The process used here was not so complicated. It became a little bit longer for we had to know which point to improve from the previous package first.

I am planning to make **NPM** Package that will include the markdown module explained here and others used for [Steadylearner][Steadylearner] also.  The API name used here might be different in the later project but the main concept will be the same.

I hope you found the post useful and can write your own plugins for your markdown also referring to the thought process used here.

**Thanks and please share this post with others**.
