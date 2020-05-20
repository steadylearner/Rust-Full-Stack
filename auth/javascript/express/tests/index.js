/* eslint-disable jest/expect-expect */
/* eslint-disable jest/no-test-callback */

const test = require("tape");
const supertest = require("supertest");

const { app } = require("../server");
const request = supertest(app);

const assert = require("assert");
const chalk = require("chalk");

test("[GET] / with async", async done => {

	const blue = chalk.blue;
	const msg = blue("Should return 200 OK");

	try {
		const req = await request
			.get("/")
			.expect(200);
		assert.strictEqual(req.text, "User visit / and verify the console to see sessionID");
	} catch(e) {
		console.log(e);
		done.fail(msg);
	}

	done.pass(msg);
	done.end();
});


