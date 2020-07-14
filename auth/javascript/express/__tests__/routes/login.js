/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
describe("Test /login", () => {
	test("[POST] without credentials. Expect validators/index.js will take control.", async done => {
		try {
			const response = await request
				.post("/login");

			const expected = { "errors": [{ "email": "Invalid value" }, { "password": "Invalid value" }] };
			expect(response.status).toBe(422);
			expect(JSON.parse(response.text)).toStrictEqual(expected);

			done();
		} catch (e) {
			done(e);
		}
	});

	test("[POST] with totally invalid credentials. Expect routes/login to take control.", async done => {
		try {
			const response = await request
				.post("/login")
				.send({
					email: "notinpostgresql@email.com",
					password: "badpasword",
				})
				.set("Content-Type", "application/json");

			expect(response.status).toBe(200);
			expect(JSON.parse(response.text).redirect).toBe("/login");

			done();
		} catch (e) {
			done(e);
		}
	});

	test("[POST] with the valid id but invalid password. Expect routes/login to take control.", async done => {
		try {
			const response = await request
				.post("/login")
				.send({
					email: "steady@learner.com",
					password: "passwor",
				});
				// .set("Content-Type", "application/json");

			expect(response.status).toBe(200);
			expect(JSON.parse(response.text).redirect).toBe("/login");

			done();
		} catch (e) {
			done(e);
		}
	});

	// $curl -X POST  http://localhost:8000/login -c cookie-file.txt -H 'Content-Type: application/json' -d '{"email":"steady@learner.com", "password":"password"}'
	test("[POST] with valid credentials. Expect routes/login to take control.", async done => {
		try {
			// Pass - $curl -d '{"email":"steady@learner.com", "password":"password"}' -H "Content-Type: application/json" -X POST http://localhost:8000/login
			// Fail - $curl -d '{"email":"steady@learner.com", "password":"password"}' -H ""Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:8000/login

			const response = await request
				.post("/login")
				.send({
					// username: Steadylearner
					email: "steady@learner.com",
					password: "password",
				})
				.set("Content-Type", "application/json"); // Pass

			expect(response.header["set-cookie"]).not.toBe(undefined);
			expect(JSON.parse(response.text).redirect).toBe("/profile/edit/Steadylearner");

			done();
		} catch (e) {
			done(e);
		}
	});
});
