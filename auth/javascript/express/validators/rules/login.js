// https://dev.to/nedsoft/a-clean-approach-to-using-express-validator-8go
const { body } = require("express-validator");

const loginValidationRules = () => {
	// Make a Postgresql schema similar to this at db/schema/users.sql
	return [
		body("email").isEmail(),
		body("password").isLength({ min: 5 }),
	];
};

module.exports = loginValidationRules;

