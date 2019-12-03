const Router = require("express-promise-router");
const router = Router();

const { index } = require("../controllers/profileController");

// https://stackoverflow.com/questions/20089582/how-to-get-a-url-parameter-in-express
// http://expressjs.com/en/api.html
// Test with /profile/Steadylearner
// router.post("/:username", async ({ isAuthenticated,params }, res) => {
router.get("/:email", index);

module.exports = router;