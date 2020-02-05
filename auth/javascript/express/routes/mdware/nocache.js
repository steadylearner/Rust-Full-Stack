// Use it to block the users to visit credential pages after logout.
// https://stackoverflow.com/questions/20429592/no-cache-in-a-nodejs-server/20429914#20429914
// browser.history.deleteAll(); in client pages.

const nocache = (_req, res, next) => {
	res.header("Cache-Control", "private, no-cache, no-store, must-revalidate");
	res.header("Expires", "-1");
	res.header("Pragma", "no-cache");
	next();
};

module.exports = nocache;