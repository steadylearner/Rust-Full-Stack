const Router = require("express-promise-router");
const router = Router();
const profile = require("../controllers/profile/profileController");
const {
	index,
	editProfile,
	deleteProfile,
}  = require("../controllers/profile/editProfileController");
const {
	updateProfileImage,
	deleteProfileImage,
} = require("../controllers/profile/profileImageController");

const nocache = require("./mdware/nocache");

router.get("/:username", profile);
router.get("/edit/:username", nocache, index);
// router.get("/edit/:username", nocache, index);

// You don't need params here because it is already included in req.user.
router.post("/edit", editProfile);
router.post("/delete", deleteProfile);

// Use Postgresql, AWS or other database and microservices instead of fs to save static files.
router.post("/image/update", updateProfileImage);
router.post("/image/delete", deleteProfileImage);

module.exports = router;
