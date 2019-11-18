const express = require("express");
const app = express();
const chalk = require("chalk");
const path = require("path");

let { PORT } = process.env;
if (PORT === undefined) {
  PORT = 8000;
}

app.use(express.static("public"));
app.set('views', path.join(__dirname, 'views'));
app.set('view engine', 'ejs');

app.get('/views', function (req, res) {
  res.render('index', { title: 'Edit this later', })
})

app.listen(PORT, () => {
  const blue = chalk.blue;
  const target = blue(`http://0.0.0.0:${PORT}`);
  console.log(`🚀 Server ready at ${target}`);
});

