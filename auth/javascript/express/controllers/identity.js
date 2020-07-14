const pg = require("../db/postgresql");
const uuid = require("uuid/v4");

// https://www.postgresql.org/docs/current/sql-update.html
const updateIdentityIdWhenLogin = async (identity_id = uuid(), email = "") => {
	const sql = "UPDATE users SET identity_id = ($1) WHERE email = ($2)";
	const query = {
		text: sql,
		values: [identity_id, email],
	};

	const { rowCount } = await pg.query(query);
	if (rowCount === 1) {
		console.log(`[Login] Could save identity_id ${identity_id} to the database.`);
	} else { // Could not save email.
		// Let users login again when they want to edit their data?
		console.log(`[Login] Couldn't save identity_id ${identity_id} to the database`);
	}
};

// Use temporaryId to prevent users to use the same identity_id when logout
// It is used instead of uuid because of the type problem it makes with Jest tests.
const updateIdentityIdWhenLogout = async (temporaryId = uuid(), identity_id = uuid()) => {
	const sql = "UPDATE users SET identity_id = ($1) WHERE identity_id = ($2)";
	const query = {
		text: sql,
		values: [temporaryId, identity_id],
	};

	const { rowCount } = await pg.query(query);
	if (rowCount === 1) {
		console.log(`[Logout] Could renew ${identity_id} to ${temporaryId}.`);
	} else { // Could not save email.
		// Let users login again when they want to edit their data?
		console.log(`[Logout] Couldn't renew ${identity_id} to ${temporaryId}.`);
	}
};

const identify = async (req) => {
	const { identity_id } = req.session;
	const sql = "SELECT * FROM users WHERE identity_id = ($1)";
	const query = {
		text: sql,
		values: [identity_id],
	};

	const { username } = req.user;

	const { rows, rowCount } = await pg.query(query);
	// console.log("[ROWCOUNT]", rowCount);
	// console.log("[EMAIL]", email);
	// console.log("[RORWS[0][EMAIL]", rows[0].email);
	if (rowCount === 0) {
		return false;
	} else { // Could not save email.
		if (username === rows[0].username) {
			return rows[0]; // user
		} else {
			return false;
		}
	}
};

// You should use this syntax to make it work because it returns Promise
// const identified = await identify(req);
// if (identified) {

module.exports = {
	updateIdentityIdWhenLogin,
	updateIdentityIdWhenLogout,
	identify,
};
