// Modulize this with prexisting project with ESLINT etc.
const express = require("express");
const app = express();
const helmet = require("helmet");
const fileUpload = require("express-fileupload");
const path = require("path");

const uuid = require("uuid/v4");

// https://github.com/expressjs/session#example
// https://www.npmjs.com/package/redis
// https://www.npmjs.com/package/connect-redis
// https://hub.docker.com/_/redis, $docker run -d -p 6379:6379 redis
// $redis-cli
const session = require("express-session");
const redis = require("redis");
const redisClient = redis.createClient();
// Use express session API and it will affect redis also.
const redisStore = require("connect-redis")(session);
const { RHOST, RPORT } = require("./db/redis");
// https://www.npmjs.com/package/session-file-store
// const FileStore = require('session-file-store')(session);

const bodyParser = require("body-parser");

// Include GitHub etc later with OAUTH2 if necessary.
const passport = require("./passport");

const {
	home,
	register,
	login,
	profile,
	password,
	authrequired,
	logout,
	//
	users,
	blog,
} = require("./routes");

const undefinedController = require("./controllers/undefinedController");

app.use(helmet());

// You can serve static files in /public and tempaltes in /views with this.
// Always be the start of your app.
app.use(express.static("public"));
app.set("views", path.join(__dirname, "views"));
app.set("view engine", "ejs");

app.use(bodyParser.urlencoded({ extended: false })); // -H application/x-www-form-urlencoded
app.use(bodyParser.json());

app.use(session({
	genid: (req) => {
		console.log("Inside the session middleware");
		console.log(req.sessionID); // undefined yet
		return uuid(); // use UUIDs for session IDs
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
	store: new redisStore({ host: RHOST, port: RPORT, client: redisClient, ttl: 86400 }),
	secret: "steady learner",
	resave: false,
	saveUninitialized: true
}));

app.use(passport.initialize());
app.use(passport.session());

// /views route will serve index.ejs, edit this later with auth relevant codes.
// app.get('/views', function (req, res) {
//   res.render('index', { title: 'Edit this later', })
// })

// https://github.com/richardgirges/express-fileupload/tree/master/example#basic-file-upload
app.use(fileUpload({
	safeFileNames: true,
	limits: { fileSize: 10 * 1024 * 1024 }, // 10mb
	abortOnLimit: true,
	// useTempFiles: true,
	// tempFileDir: "/tmp/",
}));

app.use("/", home);
app.use("/register", register);
app.use("/login", login);
app.use("/profile", profile);
app.use("/password", password);
app.use("/authrequired", authrequired);
app.use("/logout", logout);
//
app.use("/users", users);
app.use("/blog", blog);

app.use(undefinedController);

// https://stackoverflow.com/questions/52939575/node-js-jest-redis-quit-but-open-handle-warning-persists
const closeRedis = async () => {
	await new Promise((resolve) => {
		redisClient.quit(() => {
			resolve();
		});
	});
	// redis.quit() creates a thread to close the connection.
	// We wait until all threads have been run once to ensure the connection closes.
	await new Promise(resolve => setImmediate(resolve));
};

module.exports = {
	app,
	// To close it in Jest tests.
	closeRedis,
};



