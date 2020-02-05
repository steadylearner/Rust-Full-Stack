const uuid = require("uuid/v4");
const { updateIdentityIdWhenLogout } = require("./identity");
// https://chartio.com/resources/tutorials/how-to-get-all-keys-in-redis/
const index = (req, res, next) => {
	// console.log("Inside the GET /logout callback function");

	// req.session.destroy remove the sessionID from redis also because they are connected.
	// You shouldn't find it with $get req.sessionID in redis-cli after you test with this route.
	console.log(req.sessionID);
	const temporaryId = uuid();
	updateIdentityIdWhenLogout(temporaryId, req.session.identity_id);
	req.session.destroy(err => {
		if (err) {
			console.log(err);
			return next(err);
		}
		console.log(req.session); // undefined
		// Should write code to redirect user to / when session is empty also
		res.redirect("/");
	});
	// res.send(req.sessionID);
}

module.exports = {
	index
};
