// [Should] Include some log file or messages at controllers/ and models/?.

// https://www.google.com/search?client=firefox-b-d&q=how+to+show+log+node+js+npm+package
// https://www.google.com/search?client=firefox-b-d&q=how+to+use+log+with+node+js
// https://www.jstorimer.com/blogs/workingwithcode/7766119-when-to-use-stderr-instead-of-stdout
// https://nodejs.org/api/util.html#util_util_format_format_args
// https://www.npmjs.com/package/pino

const chalk = require("chalk");
const express = require('express');
const app = express();
const { expressLogger } = require("./log");

app.use(expressLogger);

const useMongo = require("./db/mongoose");
useMongo();

const {
  email,
} = require("./routes");

app.use(express.json());
app.use("/api/email/v1", email);

const PORT = 8000;

app.listen(PORT, () => {
  const blue = chalk.blue;
  const target = blue(`http://localhost:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});

// 1. Connection status handler for timeout, mongodb database stopped etc
// 2. Log to stdout or a file depending on the NODE_ENV in log.js

// if  process.env.NODE_ENV
// NODE_ENV=production node application.js, NODE_ENV=development node application.js

// 3. Tests if necessary
