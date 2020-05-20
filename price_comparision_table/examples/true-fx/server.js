// Refer to this instead of Typescript
// https://github.com/paulhoughton/fx/tree/react

// https://github.com/paulhoughton/fx/tree/webcomponents
// https://developer.mozilla.org/en-US/docs/Web/Web_Components/Using_custom_elements

var express = require('express'),
  app = express(),
  http = require('http').Server(app),
  io = require('socket.io')(http),
  fetch = require('node-fetch');

app.use(express.static(__dirname + '/'));

var fields = ["currencyPair", "timestamp", "bidBig", "bidPips", "offerBig", "offerPips", "high", "low", "open"]
var cachedData;
var connected = 0;

var interval = setInterval(updateData, 5000);

io.on('connection', function (socket) {
  connected++;
  io.sockets.emit('data', cachedData);
  socket.on('disconnect', function () {
    connected--;
  });
});

const processData = (fields, valid) => data => data.split("\n")
  .map(row => row.split(",")
  .reduce((acc, val, i) => { acc[fields[i]] = val; return acc }, {}))
  .filter(obj => obj.hasOwnProperty(valid))

function updateData() {
  fetch('http://webrates.truefx.com/rates/connect.html?f=csv')
    .then(response=>response.text())
    .then(processData(fields, "timestamp"))
    .then(result => {
      cachedData = result;
      io.sockets.emit('data', result);
  })
}

http.listen(3000, function () {
  console.log('listening on: 3000');
});


