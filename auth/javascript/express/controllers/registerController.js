const uuid = require("uuid/v4");
const bcrypt = require("bcrypt");
const pg = require("../db/postgresql");

const index = (_req, res) => {
	console.log("Inside GET /register callback function");
	res.render("register", { error: false });
};

// Refer to https://www.steadylearner.com/blog/read/How-to-use-gRPC-with-Rust-Tonic-and-Postgresql-database
const register = async ({ body }, res) => {
	console.log("Inside the POST /register callback function");
	const { username, email, password } = body;

	const sql = "INSERT INTO users(id, username, email, password) VALUES($1, $2, $3, $4)";

	const saltRounds = 10;
	const hashed = bcrypt.hashSync(password, saltRounds);

	const query = {
		text: sql,
		values: [uuid(), username, email, hashed],
	};

	try {
		const { rowCount } = await pg.query(query);
		if (rowCount === 1) {
			// When users could make an account, let them login wiht it.
			res.redirect("/login");
		} else { // Could not save email.
			res.render("register", { error: `Fail to create user with email ${email}` });
		}
	} catch (error) {
		res.render("register", { error });
	}
};

module.exports = {
	index,
	register,
};

