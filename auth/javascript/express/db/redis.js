require("dotenv").config({ path: ".env" }); // It uses releative path and called at server/user.js and server/product.js

// Use ENVIRONMENT or env-file from docker-compose.yml instead of .env
/* eslint-disable */
const {
    RHOST = "0.0.0.0",
    RPORT = 6379,
} = process.env;
/* eslint-disable */

module.exports = {
    RHOST,
    RPORT,
}
