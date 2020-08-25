// https://github.com/pinojs/pino-tee
// https://github.com/pinojs/express-pino-logger

const pino = require('pino');
const expressPino = require('express-pino-logger');

// How to log all options or just one at a time? 
// http://getpino.io/#/docs/api?id=level-string
// http://getpino.io/#/docs/api?id=opt-customlevels
const logger = pino({ level: process.env.LOG_LEVEL || 'info' });
const expressLogger = expressPino({ logger });

module.exports = {
    logger, // Used in controllers/ for debug etc.
    expressLogger, // Used in sever.js to configure Express with pino.
}

// It does work but you should make it return 
// log to stdout when "dev" and 
// write to a file when "prod". 