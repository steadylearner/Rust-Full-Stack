const fs = require("fs");
const { identify } = require("../identity");
const getProfileFolder = require("./getProfileFolder");

const saveProfileImage = (req, res) => {
	if (!req.files || Object.keys(req.files).length === 0) {
		return res.status(400).send("No files were uploaded.");
	}

	const { username } = req.user;
	let { profileImage } = req.files;

	const profileFolder = getProfileFolder();

	profileImage.mv(`${profileFolder}/${username}.png`, function (error) {
		if (error) {
			return res.status(500).send(error);
		}

		res.redirect(`/profile/${username}`);
	});
};

const updateProfileImage = async (req, res) => {
	console.log("Inside GET /profile/image/edit callback");
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			saveProfileImage(req, res);
		}
	} else {
		res.redirect("/");
	}
};

const deleteProfileImage = async (req, res) => {
	console.log("Inside GET /profile/image/delete callback");
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		console.log(req.session.identity_id);
		console.log(identified);
		if (identified === false) {
			res.redirect("/");
		} else {
			const { username } = req.user;
			console.log(username);
			const profileFolder = getProfileFolder();
			const filePath = `${profileFolder}/${username}.png`;

			try {
				fs.unlinkSync(filePath);
				const data = {
					redirect: `/profile/${username}`,
				};
				res.json(data);
			} catch(error) {
				console.log(error);
				// res.render("editPorfile");
				// res.render("editPorfile", { error, });
			}
		}
	} else {
		res.redirect("/");
	}
};

module.exports = {
	updateProfileImage,
	deleteProfileImage,
};
