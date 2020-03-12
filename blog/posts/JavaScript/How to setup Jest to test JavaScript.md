<!--
  Post {
    subtitle: "Install Jest and test some JavaScript codes.",
    image: "/code/Jest.png",
    image_decripton: "Image from its website",
    tags: "How Jest setup code",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Code]: https://github.com/steadylearner/code
[How to use JavaScript]: https://developer.mozilla.org/en/docs/Web/JavaScript
[Jest Offical Website]: https://jestjs.io/docs/en/getting-started.html
[React Prop Passer]: https://www.npmjs.com/package/prop-passer
[React Easy Markdown]: https://www.npmjs.com/package/react-easy-md

<!-- / -->

<!-- Post -->

<!-- /  -->

While I organize codes for NPM packages such as [React Prop Passer], [React Easy Markdown] and [Steadylearner Code], I found that it is easy to use **Jest** to write tests for them.

I want to share my experience and some examples used for [Steadylearner] with this post.

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use JavaScript]

2. [Jest Offical Website]

3. [Steadylearner Code]

---

I hope you already know JavaScript well. You may visit documentation from [Mozilla][How to use JavaScript] whenever you need. I also want you to visit [Jest Offical Website] and follow the instruction.

The easiest way for you to be ready to write tests for **JavaScript** and later **React** with **Jest** will be clone [Steadylearner Code] repository.

You can find codes and tests used for this website and NPM packages.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to setup Jest
2. **useShortcut** and **useRegex**
3. How to test them with Jest
4. **Conclusion**

---

<br />

## 1. How to setup Jest

I hope you already read the documentation from [its offical website][Jest Offical Website]. You should already have minimal files to test simple JavaScript codes with Jest.

Your **babel.config.js** would be similar to

```js
module.exports = {
  presets: [
    [
      '@babel/preset-env',
      {
        targets: {
          node: 'current',
        },
      },
    ],
  ],
};
```

Your **package.json** would be similar to

```json
{
  "devDependencies": {
    "@babel/core": "^7.4.0",
    "@babel/preset-env": "^7.4.2",
    "babel-jest": "^24.5.0",
    "jest": "^24.5.0"
  },
  "scripts": {
    "test": "jest"
  }
}
```

and you should have installed those dependencies with **$yarn or npm**.

Then, **JavaScript** file similar to

```js
// math.js
function sum(a, b) {
  return a + b;
}

module.exports = {
  sum,
};
```

and **Jest** code to test it

```js
// math.test.js
const { sum } = require("./math");

// 1.
describe('Examining the syntax of Jest tests', () => {
  // 2.
  test("1 + 10 eqauls to 11", () => {
    // 3.
    expect(sum(1, 10)).toBe(11);
    expect(sum(1, 10)).not.toBe(12);
  });
});
```

You could type **$yarn test** following the example and the test would pass.

What you need to know in this example are

1. Use **describe** to group **tests** before we actaully begin tests.
2. Write real codes to test what we want to verify.
3. Include tests for the same purpose inside the same **test**.

We will repeat the similar process later. We just need to modfiy it a little bit for others. It is also important to find that those codes used for tests are made with **JavaSript**.

<br />

## 2. useShortcut and useRegex

You already installed jest and made tests pass in the previous part. You may have some examples but it wouldn't be that interesting.

So I prepared JavaScript functions **useShortcut** and **useRegex**. They are used for [Steadylearner] and you can find them work in [Steadylearner Blog](https://www.steadylearner.com/blog) or [Markdown Edtior](https://www.steadylearner.com/markdown).

First, **useShortcut** function

```js
let useShortcut = (set = [["s-", "https://"]]) => (draft = "") => {
    let text = draft;
    set.forEach(value => {
        let regex = new RegExp(`: ${value[0]}`, 'g');
        text = text.replace(regex, `: ${value[1]}`);
        regex = new RegExp(`\[(]` + value[0], 'g');
        text = text.replace(regex, "(" + value[1]);
    });
    return text;
};
```

It helps you write shorcuts for links inside markdown files.

You can define set of links and there shortcuts. Then, use one of them whenever you need. Its role is similar to **useRegex** function later but is separated for it is not easy to find working codes for this purpose. You may find more usages with [React Easy Markdown] NPM package.

Before I wrote **useRegex**, I had a JavaScript function copy(textFromWeb) that you can use to copy contents from the webpages. It is similar to the code snippet below and you can also pass helper function to modify the text from the web before copy it to clipboard in your machine.

```js
function copy(
  value = "",
  fn = useRegex(),
) {
  const textField = document.createElement("textarea");

  textField.innerText = value;
  document.body.appendChild(textField);

  textField.value = fn(textField.innerHTML);

  textField.select(); // select copies html value
  document.execCommand("copy");
  textField.remove();
}
```

I wanted to share code snippets as it is but the browsers and some web frameworks sanitize them for the safety. So I need to find the way to nullify them with regex.

So I wrote **useRegex** function.

```js
let useRegex = (set = [
  [/<br\s*?>/gi, "\r\n"],
  [/&lt;/g, "<"],
  [/&gt;/g, ">"],
  [/&amp;/g, "&"],
]) => (draft = "") => {
  let text = draft;
  set.forEach(value => {
      text = text.replace(value[0], value[1]);
  });
  return text;
};
```

It helps you apply many regexes to help **copy** function we made before or can be used independently.

<br />

## 3. How to test them with Jest

For we already have codes to test ready, it is time to write tests with **Jest** for them. If you have problem, you can always visit [Steadylearner Code] repository.

Inside __tests__ folder, you can first write, **useShortcut.test.js** similar to

<!-- Use console in Websit to make it work -->

```js
const { useShortcut, reverseSet } = require("..");

describe('Test simple cases', () => {
  const set = [["s-", "https://www.steadylearner.com"]];
  const short = ": s-, (s-), s-, https://www.steadylearner.com"
  const long = ": https://www.steadylearner.com, (https://www.steadylearner.com), s-, https://www.steadylearner.com"
  test("Simply verfiy that set works with useShortcut. Values without ': ' or '('should not be affected.", () => {
    expect(useShortcut(set)(short))
      .toBe(long);
  });
  test("Same but with reverseSet to undo", () => {
    expect(useShortcut(reverseSet(set))(long))
      .toBe(short);
  });
});

describe('Test with part of .md file', () => {
  const set = [["s-", "https://www.steadylearner.com"], ["n-", "https://www.npmjs.com/package"]];
  const short = `
    # React Easy Markdown

    [Steadylearner]: s-
    [Blog]: s-/blog
    [prop-passer]: n-/prop-passer
    [Steadylearner](s-)
    [test](s-/blog)

    s-, https://www.steadylearner.com
  `
  const long = `
    # React Easy Markdown

    [Steadylearner]: https://www.steadylearner.com
    [Blog]: https://www.steadylearner.com/blog
    [prop-passer]: https://www.npmjs.com/package/prop-passer
    [Steadylearner](https://www.steadylearner.com)
    [test](https://www.steadylearner.com/blog)

    s-, https://www.steadylearner.com
  `

  test("Verfiy that set works with useShortcut .md file. Values without ': ' or '('should not be affected.", () => {
    expect(useShortcut(set)(short))
      .toBe(long);
  });
  test("Same but with reverseSet to undo", () => {
    expect(useShortcut(reverseSet(set))(long))
      .toBe(short);
  });
});
```

If you read the documentation from Jest, you already know that it tests every **.js** files inside __tests__ folder or with **.test** before its file name.

**reverseSet** here is to help you use **useShortcut** funtion in opposite direction without need to write entire set of shortcut and links. You may refer to the code snippet below if you want to include them in your test.

<!-- Use console in Websit to make it work -->

```js
const { useRegex } = require("..");

describe('Test simple cases', () => {
  const set = [
    [/<br\s*?>/gi, "\r\n"],
    [/&lt;/g, "<"],
    [/&gt;/g, ">"],
    [/&amp;/g, "&"]
  ];
  const before = "&lt;, &gt;, &amp;"
  const after = "<, >, &"
  test("Simply verfiy that set works with useRegex.", () => {
    expect(useRegex(set)(before))
      .toBe(after);
  });
});

describe('Test with part of code snippet', () => {
  const set = [
    [/<br\s*?>/gi, "\r\n"],
    [/&lt;/g, "<"],
    [/&gt;/g, ">"],
    [/&amp;/g, "&"]
  ];
  const before = `
    &lt;title&gt;React Easy Markdown&lt;/title&gt;
    &lt;p&gt;It is made with React and JavaScript by Steadylearner&lt;/p&gt;
    &amp;variable // It is for Rust
    I can't search every example of sanitization for each framework and web browsers.
    Then, I will give freedom for users to define regexp that they can use for react-easy-md.
  `
  const after = `
    <title>React Easy Markdown</title>
    <p>It is made with React and JavaScript by Steadylearner</p>
    &variable // It is for Rust
    I can't search every example of sanitization for each framework and web browsers.
    Then, I will give freedom for users to define regexp that they can use for react-easy-md.
  `

  test("Verfiy that set works with useShortcut .md file. Values without ': ' or '('should not be affected.", () => {
    expect(useRegex(set)(before))
      .toBe(after);
  });
});
```

Its structure is very similar to the previous test for **useShortcut** and you will find that many parts are same. You can test it with **$yarn test** and it will pass also. You already have some **Jest test codes** used for a real project.

<br />

## 4. Conclusion

I hope this was helpful for you to start to write tests with Jest.

It will help you

1. **Organize your code and comments**
2. **Help you write more proven code**
3. **Think about edge cases**
4. **Working examples**

and many other benefits.

If you want more Jest and other codes from [Steadylearner], you can visit [Code][Steadylearner Code] repository.

**Thanks and please share this post with others**.
