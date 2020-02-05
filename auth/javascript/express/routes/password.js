const Router = require("express-promise-router");
const router = Router();
const { index, editPassword } = require("../controllers/passwordController");

const nocache = require("./mdware/nocache");

router.get("/edit/:username", nocache, index);
router.post("/edit/", editPassword);

module.exports = router;
