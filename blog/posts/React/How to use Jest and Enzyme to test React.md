<!--
  Post {
    subtitle:  "Learn how to write tests for React with Jest and Enzyme.",
    image:  "/code/Jest_from_the_website.png",
    image_decription: "Image from its website",
    tags: "How Jest React test",
  }
-->

<!-- Link -->

[Steadylearner]: https://www.steadylearner.com
[Steadylearner Code]: https://github.com/steadylearner/code
[React]: https://reactjs.org/
[How to use JavaScript]: https://developer.mozilla.org/en/docs/Web/JavaScript
[Jest]: https://jestjs.io/docs/en/getting-started.html
[Enzyme]: https://airbnb.io/enzyme/

<!-- / -->

<!-- Steadylearner Post -->

[How to setup Jest to test JavaScript]: https://www.steadylearner.com/blog/read/How-to-setup-Jest-to-test-JavaScript

<!-- / -->

<!-- Write code for Rust CLI to read the data from the github with conditional statements and remove this comment -->

In the post previous post [How to setup Jest to test JavaScript], we learnt how to setup **Jest** to test **JavaScript**.

In this post, we will modify it and prepare the development envirionemnt to test [React] with [Jest] and [Enzyme].

<br />

<h2 class="red-white">[Prerequisite]</h2>

1. [How to use JavaScript]
2. [How to use React][React]
3. [How to setup Jest for beginners][Jest]
4. [How to use Enzyme][Enzyme]

---

You should know how to use **JavaScript** and **React** first to write codes to test them. You also need to read the official documentation from **Jest** and **Enzyme**.

You can always refer to [Steadylearner Code] repository when you have problem following this post.

If you want just the setup files and save your time, clone [it][Steadylearner Code] and modify **src** and test files in **__tests__** folder and it will work.

<br />

<h2 class="blue">Table of Contents</h2>

1. How to include **Enzyme** to test **React** with **Jest**
2. **React** code to test
3. **Jest** and **Enzyme** code for it
4. **Conclusion**

---

<br />

## 1. How to include Enzyme to test React with Jest

I hope you already read [How to setup Jest to test JavaScript]. You may use **JavaScript_Test** for the previous folder you made to test JavaScript. Then, copy it with name **React_Test** before move on.

The benfits of this are

1. You can separate test for **JavaScript** and other frameworks such as **React**.
2. You may keep starting point(**JavaScript_Test** folder) that can be modified for other frameworks.
3. **You can always use it again** whenever your setup process goes wrong.

You may use **git** or whatever you want for this purpose. The important point here is to make a backup file before you modify it.

To write test for **React**, we have to set the development environment for that.

You can do it with

```console
$yarn add react react-dom
$yarn add @babel/plugin-proposal-class-properties @babel/preset-react --dev
$yarn add enzyme enzyme-adapter-react-16 --dev
```

1. Install **react** and **react-dom**

2. Development dependencies to use ES6+ JavaScript Features

3. Packages relevant to Enzyme to test React easily

Then, Your entire **package.json** would be similar to

```json
{
  "devDependencies": {
    "@babel/core": "^7.4.0",
    "@babel/plugin-proposal-class-properties": "^7.4.0",
    "@babel/preset-env": "^7.4.2",
    "@babel/preset-react": "^7.0.0",
    "babel-jest": "^24.5.0",
    "enzyme": "^3.9.0",
    "enzyme-adapter-react-16": "^1.11.2",
    "jest": "^24.5.0"
  },
  "scripts": {
    "test": "jest"
  },
  "dependencies": {
    "react": "^16.8.5",
    "react-dom": "^16.8.5"
  }
}
```

I hope you made it. We already have the minimal dependencies to write test codes for **React** with **Jest** and **Enzyme**.

We also have to write some configuration files.

We will first modify **babel.config.js** file.

```js
module.exports = {
  presets: ['@babel/preset-env', '@babel/preset-react'],
  plugins: ['@babel/plugin-proposal-class-properties']
};
```

We just included what we installed before.

Then, we will write another configuration file for **Jest**.

```js
// jest.config.js
module.exports = {
  verbose: true,
  setupFilesAfterEnv: ["<rootDir>/src/__tests__/setup/setupTests.js"],
  testPathIgnorePatterns: [
      "<rootDir>/src/__tests__/setup/"
  ],
}
```

It is just setup files to help the **Jest** work better. We will build files and folders according to it.

We will first make ``__tests__`` folder inside **src** folder in current directory to include all the test files inside it.

Then we will build **setup** directory with **setupTests.js** inside it

```js
// seupTests.js
import { configure } from 'enzyme';
import Adapter from 'enzyme-adapter-react-16';

configure({ adapter: new Adapter() });
```

We end up building all the files required before we write codes test **React** codes with **Jest** and **Enzyme**.

You can refer to [Steadylearner Code] repository if you want to find the whole project.

<br />

## 2. React code to test

For our goal is just test it work or not, we won't need complicated examples.

We will first use [the official example](https://github.com/facebook/jest/tree/master/examples/enzyme) for **Jest with Enzyme**.

```jsx
// CheckBoxWithLabel.js
import React from 'react';

export default class CheckboxWithLabel extends React.Component {
  state = {
    isChecked: false,
  };

  onChange = () => {
    this.setState({isChecked: !this.state.isChecked});
  };

  render() {
    return (
      <label>
        <input
          type="checkbox"
          checked={this.state.isChecked}
          onChange={this.onChange}
        />
        {this.state.isChecked ? this.props.labelOn : this.props.labelOff}
      </label>
    );
  }
}
```

The code snippet above is not complicated. If you are already familiar with [React], you can easily find what this code snippet do and what matters is state of **isChecked**.

<br />

## 3. **Jest** and **Enzyme** code for it

We already have React code ready from its official example. You may save the code below inside **__tests__** folder.

```jsx
// __tests__/CheckBoxWithLabel.test.js
import React from 'react';
import { shallow } from 'enzyme';

import CheckboxWithLabel from '../CheckboxWithLabel';

it('CheckboxWithLabel changes the text after click', () => {
  // Render a checkbox with label in the document
  const checkbox = shallow(<CheckboxWithLabel labelOn="On" labelOff="Off" />);

  expect(checkbox.text()).toEqual('Off');

  checkbox.find('input').simulate('change');

  expect(checkbox.text()).toEqual('On');
});

// shallow for unit test and mount for integration test
// To test static html file use render method from enzyme
```

and `$yarn test` and verify the test result.

It will show message similar to this.

```console
Test Suites: 1 passed, 1 total
Tests:       1 passed, 1 total
Snapshots:   0 total
Ran all test suites.
```

The final folder structure of this project will be similar to this(without node_modules for simplicity)

```console
├── babel.config.js
├── jest.config.js
├── package.json
├── src
│   ├── CheckboxWithLabel.js
│   └── __tests__
│       ├── CheckboxWithLabel.test.js
│       └── setup
│           └── setupTests.js
├── yarn-error.log
└── yarn.lock
```

<br />

## 4. Conclusion

I hope you made it. It wouldn't be difficult if you read the documentations well.

If you need more examples, please refer to the [Steadylearner Code] repository.

**Thanks and please share this post with others**.
