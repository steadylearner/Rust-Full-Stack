// https://dev.to/nedsoft/a-clean-approach-to-using-express-validator-8go
const { body } = require('express-validator')

const userValidationRules = () => {
    return [
        body('email').isEmail(),
        body('password').isLength({ min: 5 }),
    ]
}

module.exports = userValidationRules;

