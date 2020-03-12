// https://node-postgres.com/api/client
// https://node-postgres.com/guides/project-structure
// https://node-postgres.com/guides/async-express

require("dotenv").config({ path: ".env" }); // It uses releative path and called at server/user.js and server/product.js

const { Client } = require("pg");

// Use ENVIRONMENT or env-file from docker-compose.yml instead of .env
/* eslint-disable */
const {
    POSTGRESQL = "postgres://postgres:postgres@localhost/auth"
} = process.env;
/* eslint-disable */

// Use Pool if necessary(https://node-postgres.com/api/pool)
const client = new Client(POSTGRESQL);
client.connect();

// You should know when to user rows or rowCount from the return value of query
module.exports = {
    query: (text, params, callback) => {
        return client.query(text, params, callback)
    },
}
