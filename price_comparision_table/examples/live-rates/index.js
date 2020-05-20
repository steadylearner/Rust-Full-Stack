const io = require('socket.io-client');
const socket = io('https://wss.live-rates.com/')

//var key = 'XXXXXXX' //YOUR LIVE-RATES SUBSCRIPTION KEY

// if you want to subscribe only specific instruments, add them to the following object, otherwise leave this commented
// var instruments = ['EURUSD', 'USDJPY', 'BTCUSD', 'ETH']

//Use the 'trial' as key to establish a 2-minute streaming connection with real-time data.
//After the 2-minute test, the server will drop the connection and block the IP for an Hour.
// { info: 'YOUR IP: xxx.xxx.xxx.xx' }
// { info: 'Trial with this IP recently used' }

const key = 'trial'

socket.on('connect', function() {
  socket.emit('key', key, (data) => {
    console.log(data); //RESPONSE FROM SERVER

    // It(data) is an object
    // https://camo.githubusercontent.com/14119f6bb9b48c25c03241fa90758376bc2c9e14/68747470733a2f2f7468756d62732e6766796361742e636f6d2f5265636b6c657373426f756e746966756c41746c616e746963626c756574616e672d73697a655f726573747269637465642e676966
    // { info: 'YOUR IP: xxx.xxx.xxx.xx' }
    // { info: 'Subscribing for 2 Minutes (Trial)' }
    // {
    //   currency: 'USDCHF',
    //   bid: 232.4,
    //   ask: 232.73,
    //   high: 236.91,
    //   low: 230.99,
    //   open: 236.66,
    //   close: 232.4,
    //   timestamp: 1582052501682
    // }
});

socket.on('rates', function(msg) {
  try {
    let obj = JSON.parse(msg);

    // Subscribe only specific rates
    if (typeof instruments === 'undefined' || instruments.length == 0) {
      console.log(obj)
    } else {
      if (instruments.includes(obj.currency)) {
        console.log(obj)
      }
    }

  } catch (e) {
    console.log(msg)
  }
});
