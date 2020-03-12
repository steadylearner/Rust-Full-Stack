const passport = require("passport");
const LocalStrategy = require("passport-local").Strategy;
const bcrypt = require("bcrypt");

const pg = require("./db/postgresql");

// configure passport.js to use the local strategy
// Refer to this https://github.com/mjhea0/passport-local-express4/blob/master/routes/index.js
passport.use(new LocalStrategy(
	{ usernameField: "email" },
	async (email, password, done) => {
		// console.log("Inside local strategy callback");

		// // here is where you make a call to the database
		// // to find the user based on their username or email address

		const sql = "SELECT * FROM users WHERE email=$1";
		const query = {
			text: sql,
			values: [email],
		};

		console.log(`\n[GET] User with email=${email}\n`);
		// https://node-postgres.com/features/queries
		try {
			const { rows } = await pg.query(query);
			if (rows.length === 0) { // There is no user with the email.
				return done(null, false, { message: "Invalid email." });
			}

			const user = rows[0];
			if (!bcrypt.compareSync(password, user.password)) {
				return done(null, false, { message: "Invalid password." });
			}
			console.log(user);
			return done(null, user);
		} catch (error) {
			console.log(error);
			return done(error);
		}
	}
));

// tell passport how to serialize the user
passport.serializeUser((user, done) => {
	// console.log("Inside serializeUser callback. User id is save to the session file store here");
	// Is this user.id from request and no need to edit more?
	done(null, user.id);
});

// https://github.com/brianc/node-postgres
// https://github.com/mjhea0/passport-local-express4
passport.deserializeUser(async (id, done) => {
	// console.log("Inside deserializeUser callback");
	// console.log(`The user id passport saved in the session file store is: ${id}`);

	const sql = "SELECT * FROM users WHERE id=$1";
	const query = {
		text: sql,
		values: [id],
	};

	try {
		console.log(`\n[GET] User with id=${id}\n`);
		const { rows } = await pg.query(query);

		// Should read more pg documenation and impove this.
		if (rows.length === 0) { // There is no user with the id.
			return done(null, false, { message: "Invalid credentials.\n" });
		}

		const user = rows[0];
		console.log(user);
		return done(null, user);
	} catch (error) {
		console.log(error);
		return done(error);
	}
});

module.exports = passport;
