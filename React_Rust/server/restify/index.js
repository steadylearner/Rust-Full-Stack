const restify = require("restify");
const chalk = require("chalk")
const PORT = 8000;

// http://restify.com/docs/server-api/#createserver
const server = restify.createServer();

const react = {
  directory: "./public",
  file: 'index.html'
}

// http://restify.com/docs/plugins-api/#servestatic
// /\/+(user)?$/
// "/((user)?$"
// code "ResourceNotFound"
// message      "/user; caused by Error: ENOENT: no such file or directory, stat 'public/user'"

server.get('/', restify.plugins.serveStatic(react));
server.get("/user", restify.plugins.serveStatic(react));

server.get("/*", restify.plugins.serveStatic({
  directory: "./public",
  default: "index.html"
}));

server.listen(PORT, "0.0.0.0", function() {
  const blue = chalk.blue;
  console.log(`${server.name} server ready at ${blue(server.url)}`);
});
