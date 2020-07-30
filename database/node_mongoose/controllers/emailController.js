// https://mongoosejs.com/docs/populate.html
// https://www.google.com/search?client=firefox-b-d&q=what+is+discriminator+in+mongoose

const Email = require('../models/Email');
const { logger } = require("../log.js");

// $curl -X GET localhost:8000/api/email/v1
const listEmails = async (_req, res) => {
    try {
        logger.info("listEmails is called");

        // https://mongoosejs.com/docs/queries.html
        // You can also use regexp?

        const items = await Email.find().sort({ date: -1 });
        res.json(items);
    } catch(e) {
        res.json({
            success: false,
        });
        logger.debug(e);
        // console.log(e);
    }
};

// Should use await when you use async and try await;
// curl -X GET localhost:8000/api/email/v1/steady@learner.com
const getEmail = async (req, res) => {
    try {
        // logger.info("getEmail is called");

        const item = await Email.findOne({
            email: req.params.email
        });
        // console.log(item)
        res.json(item);
    } catch (e) {
        res.json({
            // ...e,
            success: false,
        });
        logger.debug(e);
        // console.log(e);
    }
};


// Refer to this if you want to save various datas.
// https://mongoosejs.com/docs/validation.html#the-unique-option-is-not-a-validator

// $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
// $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
const registerEmail = async (req, res) => {
    // logger.info("registerEmail is called");

    const newEmail = new Email({
        email: req.body.email,
    });

    // Use methods here if you defined them before.
    // var fluffy = new Kitten({ name: 'fluffy' });
    // fluffy.speak(); // "Meow name is fluffy"

    try {
        const newItem = await newEmail.save();
        res.json(newItem);
    } catch(e) {
        res.json({
            // ...e,
            success: false,
        });

        logger.debug(e);
        // console.log(e);
    }
};

// Refer to this also.
// https://mongoosejs.com/docs/models.html

// $curl -X PATCH localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
const updateEmail = async (req, res) => {
    try {
        // logger.info("updateEmail is called");

        // The following updates directly into MongoDB.
        // You want to avoid them unless you want to skip the middlewares and field validations.
        // const _updatedEmail = await Email.findOneAndUpdate({ email: req.params.email }, req.body);
        // res.json({ success: true });

        const targetEmail = await Email.findOne({ email: req.params.email });
        const updatedEmail = await targetEmail.set(req.body);
        const _savedEmail = await updatedEmail.save();

        res.json({ success: true });

    } catch(e) {
        res.status(404).json({ success: false });
        
        logger.debug(e);
        // console.log(e);
    }
};

// $curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
const deleteEmail = async (req, res) => {
    try {
        // logger.info("deleteEmail is called");

        // The following updates directly into MongoDB.
        // You want to avoid them unless you want to skip the middlewares and field validations.
        // const _deletedEmail = await Email.findOneAndDelete({ email: req.params.email });

        const targetEmail = await Email.findOne({ email: req.params.email });
        const _removedEmail = await targetEmail.remove();

        res.json({ success: true });
    } catch (e) {
        res.status(404).json({ success: false });

        logger.debug(e);
        // console.log(e);
    }
};

module.exports = {
    listEmails,
    getEmail,
    registerEmail,
    updateEmail,
    deleteEmail,
}
