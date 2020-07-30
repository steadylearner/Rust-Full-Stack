const chalk = require("chalk");
const { app } = require("./server");

// Use this to deploy it with Docker and AWS
const { PORT = 8000 } = process.env; // use 80 for AWS

// Use window.location.reload(true); instead of redirects when users update their information
// Otherwise, convert frontend codes into single page apps.
app.listen(PORT, () => {
	const blue = chalk.blue;
	// 0.0.0.0 to use it easily with Docker
	const target = blue(`http://0.0.0.0:${PORT}`);
	console.log(`🚀 Express Server ready at ${target}`);
});

// Tests instead of CURL
// Comment unnecessary console.log and other codes
// Make templates for each route you have.
// Test it with Jest, SuperTest and Cypress.

// [Auth required]

// 1. Organize and improve blog routes and make views and forms for them and comments.
// 2. Read Rust email authorization blog and make it work.

// [Login System with email varification]

// 1. Make JavaScript version first. Then, decide whether you should use it more or make Rust version.
// 2. Test it with Jest, SuperTest and Cypress.
