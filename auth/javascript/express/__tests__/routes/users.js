/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

// yarn test __tests__ / routes / register.js--force - exit
describe("Test JSON Webservice at /users", () => {
	test("[GET]", async done => {
		try {
			const response = await request
				.get("/users");

			expect(response.status).toBe(200);

			done();
		} catch (e) {
			done(e);
		}
	});
});
