const index = (_req, res) => {
	// console.log("Inside the home / callback function");
	res.render("home", { title: "Steadylearner" });
};

module.exports = { index };