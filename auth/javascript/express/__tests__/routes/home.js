/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
describe("Test home at /", () => {
	test("[GET]", async done => {
		try {
			const response = await request
				.get("/");

			expect(response.status).toBe(200);
			// expect(response.text).toBe("User visit / and verify the console to see sessionID");

			done();
		} catch (e) {
			done(e);
		}
	});
});
