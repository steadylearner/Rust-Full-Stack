/* eslint-disable jest/no-test-callback */
// Its documentation is not helpful at all.
const { app, closeRedis } = require("../../server");

const supertest = require("supertest");
const request = supertest(app);

afterAll(() => closeRedis());

// Refer to this when you want to test it only.
// $yarn test __tests__/routes/register.js --force-exit
describe("Test /undefined path.", () => {
	test("[GET]", async done => {
		try {
			const undefinedPath = "/thisisundefined";
			const response = await request
				.get(undefinedPath);

			expect(response.status).toBe(404);
			expect(response.text).toBe(`${undefinedPath} is not available.`);

			done();
		} catch (e) {
			done(e);
		}
	});
});
