<!--
  Post {
    subtitle:  "Install Jest and test some JavaScript codes.",
    image:  "/code/Jest.png",
    image_decripton: "Jest Official Image",
    tags: "test JavaScript Jest Steadylearner",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[How to use JavaScript]: https://developer.mozilla.org/en/docs/Web/JavaScript
[Jest Offical Website]: https://jestjs.io/docs/en/getting-started.html

<!-- / -->

<!-- Post -->

<!-- /  -->

In the post [How to write less code for links in markdown with React][How to write less code for links in markdown with React], We have learnt how to make shortcut for links for markdown with JavaScript. It works well inside the Codesandbox example [React Markdown Improved][React Markdown Improved] without problems.

It might be ok without tests if you want to use it for your porject only. But our goal is to make a NPM package to share our improved MarkdownPreview React Module from the post.

So I will help you to learn how to setup Jest first to test JavaScript codes. We will use the code snippets we saw at [the post][How to write less code for links in markdown with React].

The main part of it was

```jsx
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
```

and our goal is to test

```md
s-/blog,
l-/steady-learner-3151b7164,
y-/UCt_jsJOe91EVjd58kHpgTfw,
t-/steadylearner_p,
g-/steadylearner,
```
become
```md
https://www.steadylearner.com/blog
https://www.linkedin.com/in/steady-learner-3151b7164/
https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw
https://twitter.com/steadylearner_p
https://github.com/steadylearner
```
or not after the main process of the post.

You may not be interested in the code snippet used here to write shorctus for markdown. But it will be help you to learn **how to setup and organize your code to test your JavaScript code**.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use JavaScript][How to use JavaScript]
2. [How to setup Jest for beginners][How to setup Jest for beginners]
3. [How to write less code for links in markdown with React][How to write less code for links in markdown with React]
4. [Steadylearner Github Repository][Steadylearner Github Repository]
---

You should know how to use JavaScript first before you test because the test codes you will use would be JavaScript Code also. We will follow the [How to setup Jest for beginners][How to setup Jest for beginners] to setup Jest for our project also. So it will be helpful for you to read and follow the page first.

Inside [Steadylearner Github Repository][Steadylearner Github Repository], It has various files relevant to tests used for this post and the site also. You may find some working examples and other useful files for you.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to setup Jest to test JavaScript
2. **How to prepare the code to test**
3. **How to write Jest** code to test it
4. **Conclusion**

---

<br />

## 1. How to setup Jest to test JavaScript

I hope you already read the documentation from [How to setup Jest for beginners][How to setup Jest for beginners]. You should already have minimal files to test JavaScript easily with Jest.

(You can refer to this part or skip this if you already knew well this topic.)


```js
// babel.config.js
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

package.json
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
You should have configuration files similar to above and have a .js file similar to code snippet below if you followed well the example given by the [example][How to setup Jest for beginners].

```js
// math.js
function sum(a, b) {
  return a + b;
}

module.exports = {
  sum,
};

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

There are no difficult points to understand those code snippets. After you made your test pass with `yarn test` following the example, What you need to know are

1. We use **describe** to group **tests** before we actaully begin tests.
2. Then we write real codes to test what we want to verify.
3. It is better to include tests for the same purpose inside the same **test**.

We will repeat the process to produce the test code in the next phase. You will find that what we should code is just to apply it for our project and repeat the similar process.

<br />

## 2. How to prepare the code to test

If you already read [the post with the code that will be here to test][How to write less code for links in markdown with React], You might have thought that the code snippets used there are not so easy to test separately.

It wouldn't be right to test the entire function just to find that what we know that it already works.  Therefore, **we will code some new functions** that does the main role to write shortcut for links in markdown.

Making them to pass the Jest test will eventually be similar to test the orignal code snippets. Let me show you the code snippet for them first. They are just the JavaScript code comparing to the code snippet you saw at the intro for this post.

What we shoud code were just JavaScript parts without React JSX syntax from the previous examples.

```jsx
// src/prefixWithReplacement.js
function prefixWithReplacement(
  href = "https://www.steadylearner.com",
  prefixWithReplacement = ["s-", "https://"],
) {
  const replaceItOrNot = href.startsWith(prefixWithReplacement[0])
  if (replaceItOrNot) {
    return `${prefixWithReplacement[1]}${href.split(prefixWithReplacement[0])[1]}`;
  } else {
    return href;
  }
}

function prefixesWithReplacements(
  href = "https://www.steadylearner.com",
  prefixesWithReplacements = [
    ["s-", "https://"],
  ],
) {
  const isHrefeIncludeAnyPrefix = prefixesWithReplacements.filter(x => href.startsWith(x[0]));

  if(isHrefeIncludeAnyPrefix.length === 1) { // === 1 instead of > 0 to be more precise
    return `${isHrefeIncludeAnyPrefix[0][1]}${href.split(isHrefeIncludeAnyPrefix[0][0])[1]}`
  } else {
    return href;
  }
}

module.exports = {
  prefixWithReplacement,
  prefixesWithReplacements,
};
```
The major functions we need to make shortcuts for markdown links were to replace **href** given by the user with the replacement we define inside **array(prefixWithReplacement)** and the **array of arrays(prefixesWithReplacments)**.

We built the functions above to test those processes we needed inside React App easily. For it is just JavaScript code without any other packages. It will serve well for the purpose of this post.

The code may not be easy for you to understand at first glance. But it will be better if you find that **prefixesWithReplacements** serve the same purpose for prefixWithReplacement.

The main difference is that the former used to define **multiple prefixes and replacements** using data format **array of arrays** instead of array.

(I thought that it would be better for you to have multiple shortcuts than to a shortcut to save our time.)

<br />

## 3. How to write Jest code to test it
For we built **the development environment to use jest** and prepared **JavaScript code** to test before, we will begin to write real **Jest code** in this part. It will not so difficult if you already knew **how to code** and **JavaScript**. For the codes **we will write will be JavaScript after all**.

Let me show you the entire Jest code first and I will explain what happend for you later.
The code snippet will be a little bit long. But it wouldn't be difficult if you already read the post before and know the purpose of this code.
```js
// src/prefixWithReplacement.test.js
// 1.
const {
  prefixWithReplacement,
  prefixesWithReplacements,
} = require("./prefixWithReplacement");

// 2.
describe('Test prefixWithReplacement when href include prefix given and vice versa', () => {
  // 3.
  const prefixWithReplacementExample = ["s-", "https://"];
  test("When href don't have the prefix in it, the test should return the original value.", () => {
    expect(prefixWithReplacement("www.steadylearner.com/blog", prefixWithReplacementExample))
      .toBe("www.steadylearner.com/blog");
  });
  test("When href have a different prefix in it, the test should return the original value.", () => {
    expect(prefixWithReplacement("h-www.steadylearner.com/blog", prefixWithReplacementExample))
      .toBe("h-www.steadylearner.com/blog");
  });
  test("When href have the same prefix in it, the test should return the replaced value.", () => {
    expect(prefixWithReplacement("s-www.steadylearner.com/blog", prefixWithReplacementExample))
      .toBe("https://www.steadylearner.com/blog");
  });
});

// 4.
describe('It should include the what the prefixWithReplacement can do also. Therefore, we test with the same arguments first.', () => {
  const prefixesWithReplacementsExample = [["s-", "https://"]];
  test("When href don't have the prefix in it, the test should return the original value.", () => {
    expect(prefixesWithReplacements("www.steadylearner.com/blog", prefixesWithReplacementsExample))
      .toBe("www.steadylearner.com/blog");
  });
  test("When href have a different prefix in it, the test should return the original value.", () => {
    expect(prefixesWithReplacements("h-www.steadylearner.com/blog", prefixesWithReplacementsExample))
      .toBe("h-www.steadylearner.com/blog");
  });
  test("When href have the same prefix in it, the test should return the replaced value.", () => {
    expect(prefixesWithReplacements("s-www.steadylearner.com/blog", prefixesWithReplacementsExample))
      .toBe("https://www.steadylearner.com/blog");
  });
});

// 5.
describe('Test when prefixesWithReplacements should return the original href', () => {
  const https = "https://";
  const jest = [
    ["j-", https],
    ["e-", https],
    ["s-", https],
    ["t-", https],
  ];
  test("When href don't have none of prefix in it, the test should return the original value.", () => {
    expect(prefixesWithReplacements("www.steadylearner.com/blog", jest))
      .toBe("www.steadylearner.com/blog");
  });
  test("When href have a different prefix from the prefixes passed to the function, the test should return the original value.", () => {
    expect(prefixesWithReplacements("h-www.steadylearner.com/blog", jest))
      .toBe("h-www.steadylearner.com/blog");
  })
});

// 6.
describe('Test when prefixesWithReplacements should return the replaced href', () => {
  test("When href have different prefixes and similar replacements in it, the test should return the replaced value.", () => {
    expect(prefixesWithReplacements("j-www.steadylearner.com", [
      ["j-", "https://"],
      ["e-", "http://"],
      ["s-", "htt://"],
      ["t-", "ht://"],
    ]))
      .toBe("https://www.steadylearner.com");
  });

  test("When href have different prefixes and the same replacement in it, the test should return the replaced value.", () => {
    const https = "https://";
    const jest = [
      ["j-", https],
      ["e-", https],
      ["s-", https],
      ["t-", https],
    ];

    expect(prefixesWithReplacements("j-www.steadylearner.com", jest))
      .toBe("https://www.steadylearner.com");
    expect(prefixesWithReplacements("e-www.steadylearner.com", jest))
      .toBe("https://www.steadylearner.com");
    expect(prefixesWithReplacements("s-www.steadylearner.com", jest))
      .toBe("https://www.steadylearner.com");
    expect(prefixesWithReplacements("t-www.steadylearner.com", jest))
      .toBe("https://www.steadylearner.com");
  });

  test("When href have diffent prefixes and replacements. It should return different replaced value.", () => {
    const example = [
      ["s-", "https://www.steadylearner.com"],
      ["l-", "https://www.linkedin.com/in"],
      ["y-", "https://www.youtube.com/channel"],
      ["t-", "https://twitter.com"],
      ["g-", "https://github.com"],
    ];

    expect(prefixesWithReplacements("s-/blog", example))
      .toBe("https://www.steadylearner.com/blog");
    expect(prefixesWithReplacements("l-/steady-learner-3151b7164", example))
      .toBe("https://www.linkedin.com/in/steady-learner-3151b7164");
    expect(prefixesWithReplacements("y-/UCt_jsJOe91EVjd58kHpgTfw", example))
      .toBe("https://www.youtube.com/channel/UCt_jsJOe91EVjd58kHpgTfw");
    expect(prefixesWithReplacements("t-/steadylearner_p", example))
      .toBe("https://twitter.com/steadylearner_p");
    expect(prefixesWithReplacements("g-/steadylearner", example))
      .toBe("https://github.com/steadylearner");
    // should return the replacement when only prefix is given to href
    expect(prefixesWithReplacements("s-", example))
      .toBe("https://www.steadylearner.com");
  });
});
```

1. First, we include the functions we built to test with Jest.
2. Test the **base function(prefixWithReplacement)** for our test. We verify that the function replace the prefix when href include it and vice versa inside the test code used here.
3. Use variables not to repeat the same code. (**It would be better to not to make them inside global scope.** You may have caution for where they are located for there scope.)
4. After we verfiy that the previous code passes, it is the time to advance it to test for multiple prefixes and replacements. For we won't use the **prefixWithReplacement** if the **prefixesWithReplacements** work, **we should verify that the same arguments we used before would work** the same with the test code for this phase with the same purpose.
5. For we could pass the test from the `4.`, what we should **test for this part is to verify that the function works for multiple prefixes**. We write some tests for the function when it should return the original href.
6. We finally get to test the code that we want to use for our project.  It won't be difficult to understand what the test does if you followed the post well to this point. The purpose of **this test is to verify the cases when the function should return replaced href** instead.

and that is all we should know to test those two functions that does the same role after all with **Jest Test Fraemwork** and to have working test code snippet for it.

<br />

## 4. Conclusion

I hope you followed examples from this post well. You may have noticed that we almost use the same code from the part `1.` to write test codes later. To advacne the example, **What we need was to know how to write JavaScript code and understand process to write tests in sequence**.

While preparing this post, I found that writing tests help you

1. **Organize your code and comments**
2. **Help you write more proven code**
3. **Think about edge cases**
4. **Working examples**

and many other benefits.

**The post and codes used here may not be perfect**. But it would help you to learn how the test framework(**Jest**) can be used to test your simple **JavaScript** codes.

You may see the source codes and contritube to posts used for this site at [Steadylearner Github Repository][Steadylearner Github Repository].

**Thanks and please share this post with others**.
