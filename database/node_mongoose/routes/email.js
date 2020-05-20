// Refer to https://github.com/mjhea0/passport-local-express4/blob/master/routes/index.js
const Router = require("express-promise-router");
const router = Router();

const {
    listEmails,
    getEmail,
    registerEmail,
    deleteEmail,
    updateEmail
} = require("../controllers/emailController");

router.get("/", listEmails);
router.get("/:email", getEmail);
router.post("/", registerEmail);
router.put("/:email", updateEmail);
router.delete("/:email", deleteEmail);

module.exports = router;
