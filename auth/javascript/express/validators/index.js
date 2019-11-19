// https://dev.to/nedsoft/a-clean-approach-to-using-express-validator-8go
const { userValidationRules } = require('./rules');
const { validationResult } = require('express-validator');

const validate = (req, res, next) => {
    const errors = validationResult(req)
    if (errors.isEmpty()) {
        return next()
    }
    const extractedErrors = []
    errors.array().map(err => extractedErrors.push({ [err.param]: err.msg }))

    return res.status(422).json({
        errors: extractedErrors,
    })
}

module.exports = {
    validate,
    userValidationRules,
}
