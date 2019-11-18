// https://medium.com/@evangow/server-authentication-basics-express-sessions-passport-and-curl-359b7456003d
const express = require("express");
const app = express();
const chalk = require("chalk");
const path = require("path");

const uuid = require('uuid/v4');
// https://github.com/expressjs/session#example
// https://www.npmjs.com/package/session-file-store
// Use these instead of session-file-store if necessary.
// https://www.npmjs.com/package/connect-pg-simple
// https://www.npmjs.com/package/connect-redis
const session = require('express-session');
const FileStore = require('session-file-store')(session);

// Use this to deploy it with Docker and AWS
let { PORT } = process.env;
if (PORT === undefined) {
  PORT = 8000;
}

// You can serve static files in /public and tempaltes in /views with this.
// Always be the start of your app.
app.use(express.static("public"));
app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'ejs');

app.use(session({
  genid: (req) => {
    console.log('Inside the session middleware')
    console.log(req.sessionID) // undefined yet
    return uuid() // use UUIDs for session IDs
  },
  store: new FileStore(),
  secret: 'steady learner',
  resave: false,
  saveUninitialized: true
}));

// /views route will serve index.ejs, edit this later with auth relevant codes.
app.get('/views', function (req, res) {
  res.render('index', { title: 'Edit this later', })
})

// $curl -X GET http://localhost:8000 -c cookie-file.txt -v
// $curl -X GET http://localhost:8000 -b cookie-file.txt -v
app.get('/', (req, res) => {
  console.log('Inside the homepage callback function');
  console.log(req.sessionID);
  res.send(`User visit / and verify the console to see sessionID`);
})

app.listen(PORT, () => {
  const blue = chalk.blue;
  // 0.0.0.0 to use it easily with Docker
  const target = blue(`http://0.0.0.0:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});

