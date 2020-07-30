const Router = require("express-promise-router"); // You can use async with it if necessary
const router = Router();

const { index } = require("../controllers/homeController");

router.get("/", index);

module.exports = router;
