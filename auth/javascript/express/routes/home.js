const { Router } = require('express');
const router = Router();

// $curl -X GET http://localhost:8000 -c cookie-file.txt -v
// $curl -X GET http://localhost:8000 -b cookie-file.txt -v
router.get('/', (req, res) => {
    console.log('Inside the homepage callback function');
    console.log(req.sessionID);
    res.send(`User visit / and verify the console to see sessionID`);
})


module.exports = router;