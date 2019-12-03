// Refer to https://github.com/mjhea0/passport-local-express4/blob/master/routes/index.js
const Router = require("express-promise-router");
const router = Router();

const { loginValidationRules, validate, } = require("../validators");
const { index, login } = require("../controllers/loginController");

router.get("/", index);
router.post("/", loginValidationRules(), validate, login);

module.exports = router;
