<!--
  Post {
    subtitle:  "Learn how to make functional component from class component in React",
    image:  "/brand/React.png",
    image_decription: "React Image from the website",
    tags: " How React component code",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Github Repository]: https://github.com/steadylearner/Steadylearner
[React Official Website]: https://reactjs.org/
[React Hook API]: https://reactjs.org/docs/hooks-intro.html
[React Spring]: https://react-spring.surge.sh/
[React Easy Markdown]: https://www.npmjs.com/package/react-easy-md
[React Marked Markdown]: https://github.com/vincent-p/react-marked-markdown#readme
[React Markdown Improved]: https://codesandbox.io/s/wz9pp1xpn8

<!-- / -->

<!-- Post -->

<!-- / -->

If you are a experienced React Developer, you can just refer to the code part and to apply it for your React code.

I wouldn't handle how to modify **state and methods etc** in class component for functional component. It will be better to read the documentation for [React Hook API] and practice it with a React Library such as [React Spring] that uses it intensively.

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [React Official Website]
2. [Class in JavaScript](https://www.digitalocean.com/community/tutorials/understanding-classes-in-javascript)
3. [Function in JavaScript](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions)

---

You should already have experience in React and know what are differences between class and function components. I hope you read the documentation given above if you want to learn more.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to turn **React class component **into functional component
2. **Conclusion**

---

<br />

## 1. How to turn React class component into functional component

For the process is very simple, it will be better to explain how to do it with an example first. We will use **MarkdownInput** class component from [React Marked Markdown].

```jsx
// MarkdownInput.js
import React from 'react';
import PropTypes from 'prop-types';

export default class MarkdownInput extends React.Component { // 1.
  render() { // 2.
    const { onChange, value, placeholder, className } = this.props; // 3.
    return (
      <textarea
        onChange={onChange}
        value={value}
        placeholder={placeholder}
        ref="textareaRef"
        className={className}
      />
    );
  }
};

MarkdownInput.propTypes = {
  onChange: PropTypes.func,
  value: PropTypes.string,
  placeholder: PropTypes.string,
  className: PropTypes.string,
};
```

In the code example above, the class component was just inheriting props from the parent component without any state or methods. 

It didn't need to be class component because it only uses props. Therfore, it will be a good example for the purpose of this post.

We will remove some parts of it first to turn it into functional component.

Let me show you the result code and explain later.

```jsx
// FunctionMarkdownInput.js
import React from 'react';
import PropTypes from 'prop-types';

function FunctionMarkdownInput ({
  onChange,
  value,
  placeholder,
  className
}) {
  return (
    <textarea
      onChange={onChange}
      value={value}
      placeholder={placeholder}
      ref="textareaRef"
      className={className}
    />
  );
}

FunctionMarkdownInput.propTypes = {
  onChange: PropTypes.func,
  value: PropTypes.string,
  placeholder: PropTypes.string,
  className: PropTypes.string,
};

export default FunctionMarkdownInput;
```

There are only a little differences between them.

What we do for that were

1. Substitue **class** with **function**  and remove **extends React.Component** from the first line. Then, move **export default** FunctionMarkdownInput to the last of the code snippet to make the funtion part more clear to understand.

2. Remove **render**  and its relevant curly bracket wrappers for we don't use **class** and no need to use it.

3. Move destructured variables **{ onChange, value, placeholder, className }** from **this.props** inside argument of **FunctionMarkdownInput** because we don't use **class ahd this** anymore.

If you already knew how **React** prop and **JavaScript** class and function work, it would be easy.

<br />

## 2. Conclusion

I hope this post helpful as a fast reference to turn React class component into function. You may use [React Hook API] after following the process used here for state and methods later.

It was first post before I prepare to write NPM [React Easy Markdown] package from [React Marked Markdown].

**Thanks and please share this post with others.**
