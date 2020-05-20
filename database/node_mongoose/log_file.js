// https://github.com/pinojs/pino-tee
// https://github.com/pinojs/express-pino-logger
// https://github.com/pinojs/express-pino-logger/issues/28

// First, $mkdir logs && cd logs && touch info.log 
const pino = require("pino")("./logs/info.log"); // getpino.io/#/docs/api
const expressPino = require('express-pino-logger');

const expressLogger = expressPino({ logger: pino });

module.exports = {
    logger: require("pino")({ level: process.env.LOG_LEVEL || 'info' }), // Used in controllers/ for debug etc.
    expressLogger, // Used in sever.js to configure Express with pino.
}

// It does work but you should make it return 
// log to stdout when "dev" and 
// write to a file when "prod". 