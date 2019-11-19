const { Router } = require('express');
const router = Router();

// $curl -X GET http://localhost:8000 -c cookie-file.txt
// $curl -X GET http://localhost:8000/authrequired -b cookie-file.txt -L
// [SERVER] User authenticated? false
// [CLIENT] Redirect to / and shows User visit / and verify the console to see sessionID

// curl -X POST  http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
// curl -X GET http://localhost:8000/authrequired -b cookie-file.txt -L
// [SERVER] User authenticated? true
// [CLIENT] you hit the authentication endpoint
router.get('/', (req, res) => {
    console.log('Inside GET /authrequired callback')
    console.log(`User authenticated? ${req.isAuthenticated()}`)
    if (req.isAuthenticated()) {
        res.send('you hit the authentication endpoint\n')
    } else {
        res.redirect('/')
    }
})

module.exports = router;