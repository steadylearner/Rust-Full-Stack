const Router = require("express-promise-router");
const router = Router();

const nocache = require("./mdware/nocache");

// curl -X POST http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'

// curl -X GET http://localhost:8000/authrequired -b cookie-file.txt -L
// [SERVER] User authenticated? true
// [CLIENT] you hit the authentication endpoint
router.get("/", nocache, (req, res) => {
	console.log("Inside GET /authrequired callback");
	console.log(`User authenticated? ${req.isAuthenticated()}`);
	if (req.isAuthenticated()) {
		res.send("you hit the authentication endpoint\n");
		// res.set("Content-Type", "text/html");
		// res.send(Buffer.from(`
		// 	<a href="/logout">Sign Out</a>
		// `));
	} else {
		res.redirect("/");
	}
});

module.exports = router;
