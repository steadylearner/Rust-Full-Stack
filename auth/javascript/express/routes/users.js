// Refer to https://github.com/mjhea0/passport-local-express4/blob/master/routes/index.js
const Router = require("express-promise-router");
const router = Router();

const { index } = require("../controllers/usersController");

router.get("/", index);

module.exports = router;
