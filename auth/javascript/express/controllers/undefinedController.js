// Include tempaltes for undefined
// https://www.codementor.io/mayowa.a/how-to-build-a-simple-session-based-authentication-system-with-nodejs-from-scratch-6vn67mcy3
// https://www.steadylearner.com/blog/read/How-to-use-Rust-Tera-for-undefined-paths

const undefinedController = ({ path }, res) => {
	res.status(404).send(`${path} is not available.`);
};

module.exports = undefinedController;