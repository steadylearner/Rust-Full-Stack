// https://github.com/expressjs/session#example
// https://www.npmjs.com/package/session-file-store
// Use these instead of session-file-store if necessary.
// https://www.npmjs.com/package/connect-pg-simple
// https://www.npmjs.com/package/connect-redis
const session = require('express-session');
const FileStore = require('session-file-store')(session);

module.export = FileStore;