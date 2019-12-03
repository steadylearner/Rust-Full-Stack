const fs = require("fs");
const toTitleCase = require("titlecase");
const pg = require("../../db/postgresql");
const { identify } = require("../identity");
const getProfileFolder = require("./getProfileFolder");

const index = async (req, res) => {
	const { username } = req.params;
	console.log(`Inside GET /profile/${username} callback`);
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	// console.log(req);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			const {
				username,
				first_name,
				last_name,
				email,
				website,
				profile_description,
			} = identified;

			const fullname = first_name && last_name && toTitleCase(`${first_name} ${last_name}`) || username;

			res.render("editProfile", {
				profile: {
					username,
					first_name,
					last_name,
					fullname,
					email,
					website,
					profile_description,
				}
			});
		}
	} else {
		res.redirect("/");
	}
};

const editProfile = async (req, res) => {
	const { username } = req.user;
	console.log(`Inside GET /profile/edit/${username} callback`);
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	// console.log(req);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			const {
				username = "",
				first_name = "null",
				last_name = "null",
				email = "",
				website = "null",
				profile_description = "null",
			} = req.body;

			const { id } = req.user;
			// console.log(req.user);
			// console.log(req.user.id);
			const sql = "UPDATE users SET username = $2, first_name = $3, last_name = $4, email = $5, website = $6, profile_description = $7 WHERE id = $1";

			const query = {
				text: sql,
				values: [id, username, first_name, last_name, email, website, profile_description],
			};

			try {
				const { rowCount } = await pg.query(query);
				if (rowCount === 1) {
					// When users could delete an account, redirect them to /
					console.log(`Could edit the ${username}.`);

					// Extract it to function?
					const oldUserName = req.user.username;
					const newUserName = req.body.username;
					if (oldUserName !== newUserName) {
						const profileFolder = getProfileFolder();

						// Edit this to async await if necessary.
						// eslint-disable-next-line node/prefer-promises/fs
						fs.rename(`${profileFolder}/${oldUserName}.png`, `${profileFolder}/${newUserName}.png`, (err) => {
							if (err) {
								console.log(err);
							}
							console.log("Rename complete!");
						});
					}

					res.redirect(`/profile/${newUserName}`);
				} else { // Could not remove the account.
					console.log(`Couldn't edit the ${username}.`);
					res.send(`Couldn't edit the ${username}.`);
					// res.render("editProfile", { error: `Fail to delete the user with username ${username}` });
				}
			} catch (error) {
				console.log(`Couldn't edit the ${username}.`);
				console.log(error);
				res.send(`Couldn't edit the ${username}.`);
				// res.render("editProfile", { error });
			}
		}
	} else {
		res.redirect("/");
	}
};

const deleteProfile = async (req, res) => {
	const { username } = req.user;
	console.log(`Inside DELETE /profile/${username} callback`);
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			const {
				id,
				username,
			} = identified;

			const sql = "DELETE FROM users WHERE id = ($1)";

			const query = {
				text: sql,
				values: [id],
			};

			try {
				const { rowCount } = await pg.query(query);
				if (rowCount === 1) {
					// When users could delete an account, redirect them to /
					console.log(`Could delete the ${username}.`);
					res.redirect("/");
				} else { // Could not remove the account.
					console.log(`Couldn't delete the ${username}.`);
					res.render("editProfile");
					// res.render("editProfile", { error: `Fail to delete the user with username ${username}` });
				}
			} catch (error) {
				console.log(`Couldn't delete the ${username}.`);
				console.log(error);
				res.render("editProfile");
				// res.render("editProfile", { error });
			}
		}
	} else {
		res.redirect("/");
	}
};

module.exports = {
	index,
	editProfile,
	deleteProfile,
};
