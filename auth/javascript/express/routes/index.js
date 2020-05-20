const home = require("./home");
const register = require("./register");
const login = require("./login");
// Should be [profile, blog, product, job] etc
const profile = require("./profile");
const password = require("./password");
const authrequired = require("./authrequired");
const logout = require("./logout");
//
const users = require("./users");
const blog = require("./blog");


module.exports = {
	home,
	register,
	login,
	profile,
	password,
	authrequired, // Remove this when you make profile work
	logout,
	//
	users,
	blog,
};