# start-server-and-test with Yarn

It uses **$npm run** command by default and not that compatible with the project used with Yarn.

## How to imporve webpack.config.json and its relevant pacakges.

1. Remove source loader from configuration and package.json.
2. Remove ESLint loader
3. include = with --mode=production
4. Use caution with React Lazy
5. Refresh cache of the browser.
6. The cypress problem was from the optimization options that makes split chunks. Write a code to not use it when it used with Cypress.
