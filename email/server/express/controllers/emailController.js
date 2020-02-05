const Email = require('../models/Email');

// $curl -X GET localhost:8000/api/email/v1
const listEmails = async (_req, res) => {
    try {
        const items = await Email.find().sort({ date: -1 });
        console.log(res.json(items));
    } catch(e) {
        res.json({
            ...e,
            success: false,
        });
        console.log(e);
    }
};

// curl -X GET localhost:8000/api/email/v1/steady@learner.com
const getEmail = async (req, res) => {
    try {
        const item = Email.findOne({
            email: req.params.email
        });
        console.log(res.json(items));
    } catch (e) {
        res.json({
            ...e,
            success: false,
        });
        console.log(e);
    }
};

// $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "steady@learner.com" }'
// $curl -X POST localhost:8000/api/email/v1 -H "Content-Type: application/json" -d '{ "email": "example@email.com" }'
const registerEmail = async (req, res) => {
    const newEmail = new Email({
        email: req.body.email,
    });

    try {
        const newItem = await newEmail.save();
        res.json(newItem);
    } catch(e) {
        res.json({
            ...e,
            success: false,
        });

        console.log(e);
    }
};

// $curl -X PUT localhost:8000/api/email/v1/steady@learner.com -H "Content-Type: application/json" -d '{ "response": "true" }'
const updateEmail = async (req, res) => {
    try {
        const _updatedItem = await Email.findOneAndUpdate({ email: req.params.email }, req.body);
        res.json({ success: true });
    } catch(e) {
        console.log(e);
        res.status(404).json({ success: false });
    }
};

// $curl -X DELETE localhost:8000/api/email/v1/steady@learner.com
const deleteEmail = async (req, res) => {
    try {
        const _deletedItem = await Email.findOneAndDelete({ email: req.params.email });
        res.json({ success: true });
    } catch (e) {
        console.log(e);
        res.status(404).json({ success: false });
    }
};

module.exports = {
    listEmails,
    getEmail,
    registerEmail,
    updateEmail,
    deleteEmail,
}
