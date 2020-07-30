const moment = require("moment");
const toTitleCase = require("titlecase");
const { identify } = require("../identity");

const profile = async (req, res) => {
	const { username } = req.params;
	console.log(`Inside GET /profile/${username} callback`);
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			const {
				username,
				// email,
				first_name,
				last_name,
				created_at,
				website,
			} = identified;

			const fullname = first_name && last_name && toTitleCase(`${first_name} ${last_name}`) || username;

			res.render("profile", {
				profile: {
					username,
					fullname,
					// email,
					created_at: moment(created_at).format("YYYY-MM-DD"),
					website,
				}
			});
		}
	} else {
		res.redirect("/");
	}
};

module.exports = profile;
