/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
const faker = require("faker");
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

const pg = require("../../db/postgresql");

afterAll(async () => {
	closeRedis();

	const sql = "DELETE FROM users WHERE NOT(email = 'steady@learner.com')";

	const query = {
		text: sql,
	};

	// It doesn't seem to be show this message. Just refer to this.
	try {
		const { rows } = await pg.query(query);
		console.log(`Delete ${rows.length} users made to test /register except default steady@learner.com account.`);
	} catch (error) {
		console.log(error);
	}
});

describe("Test /register", () => {
	test("[POST] without credentials. Expect validators/index.js will take control.", async done => {
		try {
			const response = await request
				.post("/login");

			expect(response.status).toBe(422);
			console.log(JSON.parse(response.text));

			done();
		} catch (e) {
			done(e);
		}
	});

	test("[POST] with some email and password", async done => {
		try {
			const username = faker.name.findName();
			const email = faker.internet.email();
			const response = await request
				.post("/register")
				.send({
					username,
					email,
					password: "badpasword",
				})
				.set("Content-Type", "application/json");

			expect(response.header["set-cookie"]).not.toBe(undefined);
			expect(response.header.location).toBe("/login");

			done();
		} catch (e) {
			done(e);
		}
	});

	// $curl -X POST  http://localhost:8000/register -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
	test("[POST] with prexisting account", async done => {
		try {
			const response = await request
				.post("/register")
				.send({
					username: "Steadylearner",
					email: "steady@learner.com",
					password: "password",
				})
				.set("Content-Type", "application/json");

			expect(response.status).toBe(200);
			// expect(response).toBe(true);
			// expect(response.body.detail).toStrictEqual("Key (email)=(steady@learner.com) already exists.");
			// expect(response.body.detail).toStrictEqual("duplicate key value violates unique constraint &#34;users_username_key&#34;");

			done();
		} catch (e) {
			done(e);
		}
	});
});
