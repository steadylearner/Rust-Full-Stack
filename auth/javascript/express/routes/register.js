const Router = require("express-promise-router");
const router = Router();

const { registerValidationRules, validate, } = require("../validators");
const { index, register } = require("../controllers/registerController");

router.get("/", index);
router.post("/", registerValidationRules(), validate, register);

module.exports = router;

