const pg = require("../postgresql");

const index = async ({ isAuthenticated, params }, res) => {
	console.log("Inside GET /profile callback");
	console.log(`User authenticated? ${isAuthenticated()}`);

	if (isAuthenticated()) {
		const { email } = params;
		const sql = "SELECT * FROM users WHERE email = $1";
		const query = {
			text: sql,
			values: [email],
		};

		try {
			const { rows } = await pg.query(query);

			if (rows.length !== 0) {
				console.log("\n[GET] User Profile \n");
				const user = rows[0];

				res.render("profile", { user });
			}
			else {
				res.render("register", { error: `There is no user with email ${email}.` });
			}
		} catch (error) {
			console.log(error);
			res.redirect("/login");
		}
	} else {
		res.redirect("/login");
	}

};

module.exports = {
	index,
};