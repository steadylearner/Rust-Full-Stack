const express = require("express");
const app = express();
const chalk = require("chalk");

const { PORT = 8000 } = process.env;

// const PORT = 80; 
// Use this to make a container for AWS fargate.
// $docker run -d -p 80:80 steadylearner/express:aws
// It won't work at your local mahcine directly without sudo. Make a Docker image and test it inside docker.
// $curl localhost or $curl localhost:80 or verify it at browser

app.use(express.static("public"));

app.get('/', function(req, res){
  res.sendFile("public/index.html", { root: __dirname });
});

app.get('/user', function(req, res){
  res.sendFile("public/index.html", { root: __dirname });
}); 

app.listen(PORT, () => {
  const blue = chalk.blue;
  const target = blue(`http://0.0.0.0:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});

console.log(app);
