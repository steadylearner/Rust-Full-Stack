// https://medium.com/@evangow/server-authentication-basics-express-sessions-passport-and-curl-359b7456003d

// Modulize this with prexisting project with ESLINT etc.
const express = require("express");
const app = express();
const path = require("path");

const uuid = require('uuid/v4');

// https://github.com/expressjs/session#example
// https://www.npmjs.com/package/redis
// https://www.npmjs.com/package/connect-redis
// https://hub.docker.com/_/redis, $docker run -d -p 6379:6379 redis
// $redis-cli
const session = require('express-session');
const redis = require('redis');
const redisClient = redis.createClient();
const redisStore = require('connect-redis')(session);
// https://www.npmjs.com/package/session-file-store
// const FileStore = require('session-file-store')(session);

const bodyParser = require('body-parser');

// Include GitHub etc later with OAUTH2 if necessary.
const passport = require("./passport");

// You can serve static files in /public and tempaltes in /views with this.
// Always be the start of your app.
app.use(express.static("public"));
app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'ejs');

app.use(bodyParser.urlencoded({ extended: false })); // -H application/x-www-form-urlencoded
app.use(bodyParser.json());

app.use(session({
  genid: (req) => {
    console.log('Inside the session middleware')
    console.log(req.sessionID) // undefined yet
    return uuid() // use UUIDs for session IDs
  },
  cookie: { secure: false }, // Note that the cookie-parser module is no longer needed
  // store: new FileStore(),
  // https://medium.com/mtholla/managing-node-js-express-sessions-with-redis-94cd099d6f2f
  // $redis-cli
  // $ping
  // $monitor
  // $curl localhost:8000
  // $dbsize
  // $keys *
  store: new redisStore({ host: 'localhost', port: 6379, client: redisClient, ttl: 86400 }),
  secret: 'steady learner',
  resave: false,
  saveUninitialized: true
}));

app.use(passport.initialize());
app.use(passport.session());

module.exports = app;


