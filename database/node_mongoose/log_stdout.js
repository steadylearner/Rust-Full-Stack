// https://github.com/pinojs/pino-tee
// https://github.com/pinojs/express-pino-logger

const pino = require('pino');
const expressPino = require('express-pino-logger');

const logger = pino({ level: process.env.LOG_LEVEL || 'info' });
const expressLogger = expressPino({ logger });

module.exports = {
    logger, // Used in controllers/ for debug etc.
    expressLogger, // Used in sever.js to configure Express with pino.
}
