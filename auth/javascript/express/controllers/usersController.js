const moment = require("moment");
const pg = require("../db/postgresql");

// Use this to build React or other single page apps with this logic later.
// Make a public test JSON webservice to show all the users
const index = async (_req, res) => {
	const sql = `
		SELECT username, email, created_at, website
		FROM users
		ORDER BY created_at DESC
	`;

	// Show the latest ones first with DESC

	const query = {
		text: sql,
	};

	console.log("\n[GET] Users\n");
	try {
		const { rows: users, rowLength } = await pg.query(query);
		// console.log(users);
		// Refer to ./profileController
		const payload = users.map(({
			username,
			email,
			created_at,
			website,
		}) => {
			return {
				username,
				email,
				created_at: moment(created_at).format("YYYY-MM-DD"),
				website,
			};
		});
		if (rowLength === 0) { // There is no user with the email.
			res.json({});
		}
		res.json(payload);
	} catch (error) {
		res.json({ error });
	}
};

module.exports = { index };