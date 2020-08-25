const bcrypt = require("bcrypt");
const toTitleCase = require("titlecase");
const pg = require("../db/postgresql");
const { identify } = require("./identity");

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
			} = identified;

			const fullname = first_name && last_name && toTitleCase(`${first_name} ${last_name}`) || username;

			res.render("editPassword", {
				profile: {
					username,
					fullname,
				}
			});
		}
	} else {
		res.redirect("/");
	}
};

const editPassword = async (req, res) => {
	const { username } = req.user;
	console.log(`Inside POST /password/edit/${username} callback`);
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	// console.log(req);
	if (req.isAuthenticated()) {
		const identified = await identify(req);
		if (identified === false) {
			res.redirect("/");
		} else {
			const {
				oldpassword,
				newpassword,
			} = req.body;

			const { id, password } = req.user;

			if (bcrypt.compareSync(oldpassword, password)) {
				res.send("The password you sent are not equal to what saved at the database.");
			}

			const saltRounds = 10;
			const hashedNewPassword = bcrypt.hashSync(newpassword, saltRounds);

			const sql = "UPDATE users SET password = $2 WHERE id = $1";

			const query = {
				text: sql,
				values: [id, hashedNewPassword],
			};

			try {
				const { rowCount } = await pg.query(query);
				if (rowCount === 1) {
					// When users could delete an account, redirect them to /
					console.log(`Could edit the password of the ${username}.`);
					res.redirect(`/profile/${username}`);
				} else { // Could not remove the account.
					console.log(`Couldn't edit the password of ${username}.`);
					res.send(`Couldn't edit the password of ${username}.`);
					// res.render("editProfile", { error: `Fail to delete the user with username ${username}` });
				}
			} catch (error) {
				console.log(`Couldn't edit the password of ${username}.`);
				console.log(error);
				res.send(`Couldn't edit the password of ${username}.`);
				// res.render("editProfile", { error });
			}
		}
	} else {
		res.redirect("/");
	}
};

module.exports = {
	index,
	editPassword,
};
