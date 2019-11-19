const { Router } = require('express');
const router = Router();

const { userValidationRules, validate, } = require("../validators");
const passport = require("../passport");

router.get('/', (req, res) => {
    console.log('Inside GET /login callback function')
    console.log(req.sessionID)
    // Use res.render later
    res.send(`You are at the login page! Type your id and password.\n`)
})

// curl -X POST http://localhost:8000/login -b cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
// https://www.npmjs.com/package/passport
// app.post('/login', (req, res) => {
//  console.log('Inside POST /login callback function')
//  console.log(req.body)
//  res.send(`You attempted to login with POST HTTP method and the credential datas!\n`)
// })

// Should write logic for error(fail) and redirect also.
// https://www.npmjs.com/package/passport#authenticate-requests
// $curl -X POST  http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
router.post('/', userValidationRules(), validate, (req, res, next) => {
    console.log('Inside POST /login callback')
    passport.authenticate('local', (err, user, info) => {
        console.log('Inside passport.authenticate() callback');
        console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`)
        console.log(`req.user: ${JSON.stringify(req.user)}`)
        req.login(user, (err) => {
            console.log('Inside req.login() callback')
            console.log(`req.session.passport: ${JSON.stringify(req.session.passport)}`)
            console.log(`req.user: ${JSON.stringify(req.user)}`)
            // Use res.render("profile");
            return res.send('You were authenticated & logged in!\n');
        })
    })(req, res, next);
})

module.exports = router;