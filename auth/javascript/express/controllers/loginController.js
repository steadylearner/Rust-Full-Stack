const passport = require("../passport");
// const pg = require("../db/postgresql");
const uuid = require("uuid/v4");
const { updateIdentityIdWhenLogin } = require("./identity");

const index = (req, res) => {
	// console.log("Inside GET /login callback function");
	console.log(req.sessionID);
	res.render("login", { error: false });
};

// Should write logic for error(fail) and redirect also.
// https://www.npmjs.com/package/passport#authenticate-requests
// You can only use async await when the API return Promise.
// This passport NPM package doesn't support it yet.
const login = (req, res, next) => {
	// console.log("Inside the POST /login callback function");
	// (err, user, info) is equal to done(null, false, { message: 'Invalid credentials.\n' }); in passport.js
	passport.authenticate("local", { failureRedirect: "/login", failureFlash: true }, (err, user, info) => {
		// console.log("Inside passport.authenticate() callback");
		// console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`);
		// console.log(`req.user: ${JSON.stringify(req.user)}`);
		console.log(err);

		req.login(user, async (error) => { // error === fail to login
			// console.log("Inside req.login() callback");
			console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`);
			console.log(`req.user: ${JSON.stringify(req.user)}`);
			if (error) {
				const data = {
					redirect: "/login",
					error
				};
				res.json(data);
			} else {
				if (user !== false) {
					const identity_id = uuid();
					req.session.identity_id = identity_id;

					updateIdentityIdWhenLogin(identity_id, user.email);

					console.log("[LOGIN] - Could login and send user to /authrequired");
					console.log(user);
					const data = {
						redirect: `/profile/edit/${user.username}`,
					};
					// Temporary solution
					console.log(req.session);
					res.json(data);
				}

				if (info) {
					const data = {
						redirect: "/login",
						error: info,
					};
					res.json(data);
				}
			}
		});
	})(req, res, next);
};

module.exports = {
	index,
	login,
};

// req.session.identity_id;

// www.medium.com gives random identity_id beside session_id;
// Save this to sql and compare when users edit their datas?
// "identity_id": "726713137256137487",
// random uuid and save it to the database also?
// the problem here is you can also visit the pages of other loged in users
// req.session.identity = uuid(); // save it to the database also?
// req.session.identity = uuid();
// console.log(req.session.passport.user);
// console.log("[RESPONSE]", res);
// Save res.sessionID here to database and compare when users edit there blog, proifle etc?
// d072aa0e-aeaf-41d3-ac39-d6efa364b882