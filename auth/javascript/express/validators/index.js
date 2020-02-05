// https://dev.to/nedsoft/a-clean-approach-to-using-express-validator-8go
const { validationResult } = require("express-validator");
const {
	registerValidationRules,
	loginValidationRules,
} = require("./rules");

const validate = (req, res, next) => {
	const errors = validationResult(req);
	if (errors.isEmpty()) {
		return next();
	}
	const extractedErrors = [];
	errors.array().map(err => extractedErrors.push({ [err.param]: err.msg }));

	// console.log("[Validate REQUEST]", req);
	// console.log("[Validate RESPONSE]", res);

	return res.status(422).json({
		errors: extractedErrors,
	});
	// res.render("login", { error: extractedErrors });
};

module.exports = {
	validate,
	registerValidationRules,
	loginValidationRules,
};
